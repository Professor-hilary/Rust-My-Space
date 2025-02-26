import numpy as np

class SimpleEnv:
    def __init__(self):
        self.states = 10
        self.actions = 2
        self.state = np.random.randint(0, self.states)

    def step(self, action):
        reward = np.random.choice([1, -1])  # Reward is random
        next_state = (self.state + action) % self.states
        self.state = next_state
        return next_state, reward

    def reset(self):
        self.state = np.random.randint(0, self.states)
        return self.state
