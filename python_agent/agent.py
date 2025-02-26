import numpy as np
from rusty_core import QLearning

class Agent:
    def __init__(self, state_size, action_size, lr=0.1, gamma=0.9, epsilon=0.1):
        self.q_learning = QLearning(state_size, action_size, lr, gamma, epsilon)

    def act(self, state):
        return self.q_learning.choose_action(state)

    def learn(self, state, action, reward, next_state):
        self.q_learning.update(state, action, reward, next_state)
