from agent import Agent
from environment import SimpleEnv

env = SimpleEnv()
agent = Agent(state_size=10, action_size=2)

# Training loop
for episode in range(100):
    state = env.reset()
    total_reward = 0

    for step in range(50):
        action = agent.act(state)
        next_state, reward = env.step(action)
        agent.learn(state, action, reward, next_state)
        total_reward += reward
        state = next_state

    print(f"Episode {episode + 1}: Total Reward = {total_reward}")
