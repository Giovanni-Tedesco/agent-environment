use super::abstract_game::environment::Environment;


fn minmax_search<Action, AgentId, T>(
    env: &T,
    agent: &AgentId,
    reward: impl Fn(&T, &AgentId) -> f64,
    depth: u8
) -> f64
where
AgentId: Eq,
T: Environment<Action, AgentId> + Copy + Clone
{
    if env.is_terminal() | (depth == 0) {
        return reward(env, agent);
    } else {
        let mut new_env;
        let mut new_agent: AgentId;
        let new_depth = depth - 1;

        let actions = env.valid_actions(agent);
        
        let mut best_score = f64::NEG_INFINITY;
        let mut current_score;

        for action in actions {
            new_env = env.clone();
            new_env.update(agent, &action);
            new_agent = new_env.turn();

            current_score = minmax_search(&new_env, &new_agent, &reward, new_depth);

            if current_score > best_score {
                best_score = current_score;
            } else {};
        }
        
        return best_score;
    }
}