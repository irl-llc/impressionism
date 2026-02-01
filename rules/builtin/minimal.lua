-- Minimal ruleset for skill activation.
--
-- This ruleset provides the most conservative approach to skill activation.
-- Skills are only activated on explicit user request or very high confidence.

local M = {}

--- Evaluate which skills should be activated.
-- @param context Table containing session_id, recent_message, hook_type, tool_name
-- @return Array of skill names to activate (always empty for minimal)
function M.evaluate_activation(context)
    -- Minimal ruleset never auto-activates skills
    -- Skills must be activated through explicit user request
    return {}
end

--- Evaluate which skills should be deactivated.
-- @param context Table containing session_id, recent_message, hook_type, tool_name
-- @return Array of skill names to deactivate
function M.evaluate_deactivation(context)
    -- Deactivate all skills on session stop
    if context.hook_type == "stop" then
        local active = impressionism.get_active_skills(context.session_id)
        local names = {}
        for _, skill in ipairs(active) do
            table.insert(names, skill.name)
        end
        return names
    end

    return {}
end

return M
