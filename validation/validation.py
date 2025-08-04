import csv
import matplotlib.pyplot as plt
import control as ct
from control import tf, step_response, ss
import numpy as np


def third_order_system(time):
    # Define third-order system: P(s) = 1 / (s^3 + 6s^2 + 11s + 6)
    A = np.array([[0, 1, 0], [0, 0, 1], [-6, -11, -6]])
    B = np.array([[0], [0], [1]])
    C = np.array([[1, 0, 0]])
    D = np.array([[0]])
    plant = ss(A, B, C, D)

    # Define PID controller: C(s) = Kp + Ki/s + Kd*s
    # That becomes: (Kd*s^2 + Kp*s + Ki) / s
    Kp = 25.0
    Ki = 0.0
    Kd = 0.0
    pid_numerator = [Kd, Kp, Ki]
    pid_denominator = [1, 0]
    pid = tf(pid_numerator, pid_denominator)

    # Open-loop system: G(s) = C(s) * P(s)
    open_loop = pid * plant

    # Closed-loop system with unity feedback
    closed_loop = ct.feedback(open_loop, 1)

    T = np.linspace(0, time[-1], len(time))
    return step_response(closed_loop, T)


def rl_circuit_open(time):
    # Define third-order system: P(s) = 1 / (0.05s + 5)
    # plant = tf([1.0], [0.05, 5.0])
    A = np.array([[-100]])
    B = np.array([[1]])
    C = np.array([[20]])
    D = np.array([[0]])
    plant = ss(A, B, C, D)

    T = np.linspace(0, time[-1], len(time))
    return step_response(plant, T)


def rl_circuit_closed(time):
    # Define third-order system: P(s) = 1 / (0.05s + 5)
    plant = tf([1.0], [0.05, 5.0])

    # Define PID controller: C(s) = Kp + Ki/s + Kd*s
    # That becomes: (Kd*s^2 + Kp*s + Ki) / s
    Kp = 1.0
    Ki = 0.0
    Kd = 0.0
    pid_numerator = [Kd, Kp, Ki]
    pid_denominator = [1, 0]
    pid = tf(pid_numerator, pid_denominator)

    # Open-loop system: G(s) = C(s) * P(s)
    open_loop = pid * plant

    # Closed-loop system with unity feedback
    closed_loop = ct.feedback(open_loop, 1)

    T = np.linspace(0, time[-1], len(time))
    return step_response(closed_loop, T)


def load_aule_simulated_data(filename: str):
    data = np.loadtxt(filename, delimiter=",", skiprows=1)
    time = data[:, 0]
    output = data[:, 1]

    return time, output


def plot_both_responses(time, output, ref_time, ref_output, title):
    plt.plot(ref_time, ref_output, "k--", linewidth=2)
    plt.plot(time, output, "r", linewidth=2)
    plt.title(title)
    plt.xlabel("time (s)")
    plt.ylabel("output")
    plt.legend(["Reference", "Output"], loc="best")
    plt.grid(True)


plt.rcParams["axes.grid"] = True
plt.figure()

plt.subplot(3, 1, 1)
time_3rd_order, output_3rd_order = load_aule_simulated_data(
    "output/third_order_system.csv"
)
ref_time_3rd_order, ref_3rd_order_output = third_order_system(time_3rd_order)
plot_both_responses(
    time_3rd_order,
    output_3rd_order,
    ref_time_3rd_order,
    ref_3rd_order_output,
    "Third Order System Step Response",
)

plt.subplot(3, 1, 2)
time_rl_open, output_rl_open = load_aule_simulated_data(
    "output/open_loop_rl_circuit.csv"
)
ref_time_rl_open, ref_rl_open_output = rl_circuit_open(time_rl_open)
plot_both_responses(
    time_rl_open,
    output_rl_open,
    ref_time_rl_open,
    ref_rl_open_output,
    "Open Loop RL Circuit Step Response",
)

plt.subplot(3, 1, 3)
time_rl_closed, output_rl_closed = load_aule_simulated_data(
    "output/closed_loop_rl_circuit.csv"
)
ref_time_rl_closed, ref_rl_closed_output = rl_circuit_closed(time_rl_open)
plot_both_responses(
    time_rl_closed,
    output_rl_closed,
    ref_time_rl_closed,
    ref_rl_closed_output,
    "Closed Loop RL Circuit Step Response",
)

plt.show()
