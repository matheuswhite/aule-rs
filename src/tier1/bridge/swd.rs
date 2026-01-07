#[cfg(feature = "swd")]
type BridgeId = [u8; 6];

#[cfg(all(feature = "std", feature = "swd"))]
pub mod std {
    use crate::tier1::bridge::swd::BridgeId;
    use crate::{block::Block, signal::Signal};
    use core::marker::PhantomData;
    use num_traits::{FromBytes, ToBytes};
    use probe_rs::probe::WireProtocol;
    use probe_rs::{Core, MemoryInterface, Session, SessionConfig};
    use std::vec;
    use std::{
        collections::HashMap,
        eprintln,
        string::{String, ToString},
        sync::mpsc::{Receiver, Sender, channel},
        thread::{self},
        vec::Vec,
    };

    enum SwdMessage {
        DownReq { name: BridgeId, data: Vec<u8> },
        UpReq { name: BridgeId, size: usize },
        UpRsp { data: Vec<u8> },
    }

    struct RspCtx {
        name: BridgeId,
        sender: Option<Sender<SwdMessage>>,
    }

    pub struct SwdConnection {
        req_sender: Sender<SwdMessage>,
        rsp_sender_sender: Sender<RspCtx>,
        bridges: Vec<BridgeId>,
    }

    impl SwdConnection {
        pub fn new(chip_name: &str, core: usize, ram_offset: u64, ram_size: u64) -> Self {
            let (req_sender, req_recv) = channel();
            let (rsp_sender_sender, rsp_sender_recv) = channel();
            let cfg = SessionConfig {
                speed: Some(8_000),
                protocol: Some(WireProtocol::Swd),
                ..Default::default()
            };
            let session = Session::auto_attach(chip_name, cfg).unwrap();

            thread::spawn(move || {
                Self::task(
                    session,
                    core,
                    ram_offset,
                    ram_size,
                    req_recv,
                    rsp_sender_recv,
                )
            });

            Self {
                req_sender,
                rsp_sender_sender,
                bridges: Vec::new(),
            }
        }

        fn task(
            mut session: Session,
            core: usize,
            ram_offset: u64,
            ram_size: u64,
            req_recv: Receiver<SwdMessage>,
            rsp_sender_recv: Receiver<RspCtx>,
        ) {
            let mut bridge_table: HashMap<BridgeId, (u64, Option<Sender<SwdMessage>>)> =
                HashMap::new();
            let mut core = session.core(core).unwrap();

            loop {
                if let Ok(RspCtx { name, sender }) = rsp_sender_recv.try_recv() {
                    let Some(address) = find_address(&mut core, name, ram_offset, ram_size) else {
                        eprintln!("Fail to find {:?}", name);
                        continue;
                    };

                    bridge_table.insert(name, (address, sender));
                }

                if let Ok(msg) = req_recv.try_recv() {
                    match msg {
                        SwdMessage::DownReq { name, mut data } => {
                            let (address, _sender) = bridge_table.get(&name).unwrap();

                            let ready_address = address + 6;
                            let data_address = address + 8;

                            data.reverse();
                            core.write(data_address, data.as_slice()).unwrap();
                            core.write_8(ready_address, &[1]).unwrap();
                        }
                        SwdMessage::UpReq { name, size } => {
                            let (address, sender) = bridge_table.get(&name).unwrap();

                            let ready_address = address + 6;
                            let data_address = address + 8;

                            let mut ready = [0u8; 1];
                            loop {
                                core.read_8(ready_address, &mut ready).unwrap();
                                if ready[0] != 0 {
                                    break;
                                }
                            }

                            let mut output = vec![0u8; size];
                            core.read(data_address, output.as_mut_slice()).unwrap();

                            sender
                                .as_ref()
                                .unwrap()
                                .send(SwdMessage::UpRsp { data: output })
                                .unwrap();

                            core.write_8(ready_address, &[0]).unwrap();
                        }
                        _ => unreachable!(),
                    }
                }
            }
        }

        fn build_real_name(name: &str, is_down: bool) -> [u8; 6] {
            let mut real_name = [0u8; 6];

            real_name[0] = b'$';
            real_name[1] = if is_down { b'!' } else { b'@' };
            for (i, b) in name.as_bytes().iter().take(4).enumerate() {
                real_name[2 + i] = *b;
            }

            real_name
        }

        pub fn new_bridge_down<T, const N: usize>(
            &mut self,
            name: &str,
        ) -> Result<BridgeSwdDown<T, N>, String>
        where
            T: Clone + ToBytes<Bytes = [u8; N]>,
        {
            let real_name = Self::build_real_name(name, true);

            if self.bridges.contains(&real_name) {
                return Err("A bridge with this name has already taken".to_string());
            }

            self.rsp_sender_sender
                .send(RspCtx {
                    name: real_name,
                    sender: None,
                })
                .unwrap();

            Ok(BridgeSwdDown::new(self.req_sender.clone(), real_name))
        }

        pub fn new_bridge_up<T, const N: usize>(
            &mut self,
            name: &str,
        ) -> Result<BridgeSwdUp<T, N>, String>
        where
            T: Clone + FromBytes<Bytes = [u8; N]>,
        {
            let real_name = Self::build_real_name(name, false);

            if self.bridges.contains(&real_name) {
                return Err("A bridge with this name has already taken".to_string());
            }

            let (req, rsp) = channel();
            self.rsp_sender_sender
                .send(RspCtx {
                    name: real_name,
                    sender: Some(req),
                })
                .unwrap();

            Ok(BridgeSwdUp::new(self.req_sender.clone(), rsp, real_name))
        }

        pub fn new_remote_block<T, const N: usize>(
            &mut self,
            name: &str,
        ) -> Result<RemoteSwd<T, N>, String>
        where
            T: Clone + ToBytes<Bytes = [u8; N]> + FromBytes<Bytes = [u8; N]>,
        {
            Ok(RemoteSwd::new(
                self.new_bridge_down(name)?,
                self.new_bridge_up(name)?,
            ))
        }
    }

    fn find_address(
        probe: &mut Core,
        name: [u8; 6],
        ram_offset: u64,
        ram_size: u64,
    ) -> Option<u64> {
        const CHUNK_SIZE: usize = 0x1000;
        let num_chunks = ram_size / CHUNK_SIZE as u64;
        let mut buffer = [0u8; CHUNK_SIZE];

        for i in 0..num_chunks {
            let address = i * CHUNK_SIZE as u64 + ram_offset;
            if probe.read(address, &mut buffer).is_err() {
                continue;
            }

            for j in 0..(CHUNK_SIZE - 6) {
                let id_bytes = &buffer[j..j + 6];
                if id_bytes == name {
                    return Some(address + j as u64);
                }
            }
        }

        None
    }

    pub struct BridgeSwdDown<T, const N: usize>
    where
        T: Clone + ToBytes<Bytes = [u8; N]>,
    {
        req: Sender<SwdMessage>,
        name: BridgeId,
        _marker: PhantomData<T>,
    }

    impl<T, const N: usize> BridgeSwdDown<T, N>
    where
        T: Clone + ToBytes<Bytes = [u8; N]>,
    {
        fn new(req: Sender<SwdMessage>, name: BridgeId) -> Self {
            Self {
                req,
                name,
                _marker: PhantomData,
            }
        }
    }

    impl<T, const N: usize> Block for BridgeSwdDown<T, N>
    where
        T: Clone + ToBytes<Bytes = [u8; N]>,
    {
        type Input = T;
        type Output = ();

        fn output(&mut self, input: Signal<Self::Input>) -> Signal<Self::Output> {
            let slice_data = input.value.to_be_bytes();

            self.req
                .send(SwdMessage::DownReq {
                    name: self.name,
                    data: slice_data.to_vec(),
                })
                .unwrap();

            input.map(|_| ())
        }
    }

    pub struct BridgeSwdUp<T, const N: usize>
    where
        T: Clone + FromBytes<Bytes = [u8; N]>,
    {
        req: Sender<SwdMessage>,
        rsp: Receiver<SwdMessage>,
        name: BridgeId,
        _marker: PhantomData<T>,
    }

    impl<T, const N: usize> BridgeSwdUp<T, N>
    where
        T: Clone + FromBytes<Bytes = [u8; N]>,
    {
        fn new(req: Sender<SwdMessage>, rsp: Receiver<SwdMessage>, name: BridgeId) -> Self {
            Self {
                req,
                rsp,
                name,
                _marker: PhantomData,
            }
        }
    }

    impl<T, const N: usize> Block for BridgeSwdUp<T, N>
    where
        T: Clone + FromBytes<Bytes = [u8; N]>,
    {
        type Input = ();
        type Output = T;

        fn output(&mut self, input: Signal<Self::Input>) -> Signal<Self::Output> {
            self.req
                .send(SwdMessage::UpReq {
                    name: self.name,
                    size: size_of::<T>(),
                })
                .unwrap();

            let SwdMessage::UpRsp { data } = self.rsp.recv().unwrap() else {
                unreachable!()
            };

            let mut data_slice = [0u8; N];
            data_slice.clone_from_slice(&data);
            let output = T::from_le_bytes(&data_slice);
            input.map(|_| output)
        }
    }

    pub struct RemoteSwd<T, const N: usize>
    where
        T: Clone + ToBytes<Bytes = [u8; N]> + FromBytes<Bytes = [u8; N]>,
    {
        down: BridgeSwdDown<T, N>,
        up: BridgeSwdUp<T, N>,
    }

    impl<T, const N: usize> RemoteSwd<T, N>
    where
        T: Clone + ToBytes<Bytes = [u8; N]> + FromBytes<Bytes = [u8; N]>,
    {
        fn new(down: BridgeSwdDown<T, N>, up: BridgeSwdUp<T, N>) -> Self {
            Self { down, up }
        }
    }

    impl<T, const N: usize> Block for RemoteSwd<T, N>
    where
        T: Clone + ToBytes<Bytes = [u8; N]> + FromBytes<Bytes = [u8; N]>,
    {
        type Input = T;
        type Output = T;

        fn output(&mut self, input: Signal<Self::Input>) -> Signal<Self::Output> {
            let down_output = self.down.output(input);
            self.up.output(down_output)
        }
    }
}

#[cfg(all(not(feature = "std"), feature = "swd"))]
pub mod no_std {
    use crate::{block::Block, signal::Signal, tier1::bridge::swd::BridgeId};
    use alloc::vec::Vec;
    use core::ptr;

    #[derive(Default)]
    pub struct SwdConnection {
        bridges: Vec<BridgeId>,
    }

    #[derive(Debug)]
    pub enum SwdError {
        BridgeHasAlreadyTaken,
    }

    impl SwdConnection {
        fn build_real_name(name: &str, is_down: bool) -> [u8; 6] {
            let mut real_name = [0u8; 6];

            real_name[0] = b'$';
            real_name[1] = if is_down { b'!' } else { b'@' };
            for (i, b) in name.as_bytes().iter().take(4).enumerate() {
                real_name[2 + i] = *b;
            }

            real_name
        }

        pub fn new_bridge_down<T>(&mut self, name: &str) -> Result<BridgeSwdDown<T>, SwdError>
        where
            T: Default,
        {
            let real_name = Self::build_real_name(name, true);

            if self.bridges.contains(&real_name) {
                return Err(SwdError::BridgeHasAlreadyTaken);
            }

            Ok(BridgeSwdDown::new(real_name))
        }

        pub fn new_bridge_up<T>(&mut self, name: &str) -> Result<BridgeSwdUp<T>, SwdError>
        where
            T: Default,
        {
            let real_name = Self::build_real_name(name, false);

            if self.bridges.contains(&real_name) {
                return Err(SwdError::BridgeHasAlreadyTaken);
            }

            Ok(BridgeSwdUp::new(real_name))
        }

        pub fn new_remote_block<T>(&mut self, name: &str) -> Result<RemoteSwd<T>, SwdError>
        where
            T: Default,
        {
            Ok(RemoteSwd::new(
                self.new_bridge_down(name)?,
                self.new_bridge_up(name)?,
            ))
        }
    }

    #[repr(C)]
    #[repr(align(1))]
    pub struct BridgeSwdDown<T>
    where
        T: Default,
    {
        id: BridgeId,
        ready: bool,
        data: T,
    }

    impl<T> BridgeSwdDown<T>
    where
        T: Default,
    {
        pub fn new(name: BridgeId) -> Self {
            Self {
                id: name,
                ready: false,
                data: T::default(),
            }
        }

        pub fn layout(&self) -> [usize; 3] {
            [
                &self.id as *const _ as usize,
                &self.ready as *const _ as usize,
                &self.data as *const _ as usize,
            ]
        }
    }

    impl<T> Block for BridgeSwdDown<T>
    where
        T: Default,
    {
        type Input = ();
        type Output = T;

        fn output(&mut self, input: Signal<Self::Input>) -> Signal<Self::Output> {
            let ready_ptr: *mut bool = &mut self.ready;
            let data_ptr: *mut T = &mut self.data;

            loop {
                // TODO: Write down why this is safe
                let ready = unsafe { ptr::read_volatile(ready_ptr) };
                if ready {
                    break;
                }
            }
            unsafe {
                ptr::write_volatile(ready_ptr, false);
            }

            // TODO: Write down why this is safe
            let output = unsafe { ptr::read_volatile(data_ptr) };

            input.map(|_| output)
        }
    }

    #[repr(C)]
    #[repr(align(1))]
    pub struct BridgeSwdUp<T>
    where
        T: Default,
    {
        id: BridgeId,
        ready: bool,
        data: T,
    }

    impl<T> BridgeSwdUp<T>
    where
        T: Default,
    {
        pub fn new(name: BridgeId) -> Self {
            Self {
                id: name,
                ready: false,
                data: T::default(),
            }
        }
    }

    impl<T> Block for BridgeSwdUp<T>
    where
        T: Default + Clone,
    {
        type Input = T;
        type Output = ();

        fn output(&mut self, input: Signal<Self::Input>) -> Signal<Self::Output> {
            self.data = input.value.clone();
            self.ready = true;

            input.map(|_| ())
        }
    }

    #[repr(C)]
    pub struct RemoteSwd<T>
    where
        T: Default,
    {
        down: BridgeSwdDown<T>,
        up: BridgeSwdUp<T>,
    }

    impl<T> RemoteSwd<T>
    where
        T: Default,
    {
        pub fn new(down: BridgeSwdDown<T>, up: BridgeSwdUp<T>) -> Self {
            Self { down, up }
        }
    }

    impl<T> Block for RemoteSwd<T>
    where
        T: Default + Clone,
    {
        type Input = T;
        type Output = T;

        fn output(&mut self, input: Signal<Self::Input>) -> Signal<Self::Output> {
            let up_output = self.up.output(input);
            self.down.output(up_output)
        }
    }
}
