-- Default ruleset for skill activation.
--
-- This ruleset provides balanced skill activation based on message content
-- and session context. It uses semantic search to find relevant skills.

local M = {}

-- Configuration parameters (can be overridden via impressionism.get_param)
local config = {
    activation_threshold = 0.7,    -- Minimum similarity for activation
    max_active_skills = 5,         -- Maximum concurrent active skills
    search_limit = 10,             -- Number of candidates to consider
    deactivation_threshold = 0.3,  -- Below this, consider deactivation
}

--- Evaluate which skills should be activated.
-- @param context Table containing session_id, recent_message, hook_type, tool_name
-- @return Array of skill names to activate
function M.evaluate_activation(context)
    local skills_to_activate = {}

    -- On session start, activate based on any initial context
    if context.hook_type == "session_start" then
        return skills_to_activate
    end

    -- Skip if no message content to analyze
    if not context.recent_message then
        return skills_to_activate
    end

    -- Search for relevant skills based on message content
    local threshold = impressionism.get_param("activation_threshold", config.activation_threshold)
    local limit = impressionism.get_param("search_limit", config.search_limit)
    local max_skills = impressionism.get_param("max_active_skills", config.max_active_skills)

    local candidates = impressionism.search_skills(context.recent_message, limit)
    local active = impressionism.get_active_skills(context.session_id)
    local active_count = #active

    for _, candidate in ipairs(candidates) do
        -- Skip if already active
        if is_active(candidate.name, active) then
            goto continue
        end

        -- Skip if we've reached max active skills
        if active_count >= max_skills then
            break
        end

        -- Activate if similarity exceeds threshold
        if candidate.similarity >= threshold then
            table.insert(skills_to_activate, candidate.name)
            active_count = active_count + 1
        end

        ::continue::
    end

    return skills_to_activate
end

--- Evaluate which skills should be deactivated.
-- @param context Table containing session_id, recent_message, hook_type, tool_name
-- @return Array of skill names to deactivate
function M.evaluate_deactivation(context)
    local skills_to_deactivate = {}

    -- On stop, deactivate all skills
    if context.hook_type == "stop" then
        local active = impressionism.get_active_skills(context.session_id)
        for _, skill in ipairs(active) do
            table.insert(skills_to_deactivate, skill.name)
        end
        return skills_to_deactivate
    end

    -- Skip deactivation check if no message to analyze
    if not context.recent_message then
        return skills_to_deactivate
    end

    -- Check if any active skills are no longer relevant
    local threshold = impressionism.get_param("deactivation_threshold", config.deactivation_threshold)
    local active = impressionism.get_active_skills(context.session_id)

    if #active == 0 then
        return skills_to_deactivate
    end

    -- Embed the recent message for comparison
    local message_embedding = impressionism.embed_text(context.recent_message)
    if #message_embedding == 0 then
        return skills_to_deactivate
    end

    for _, skill in ipairs(active) do
        -- Skip skills without embeddings
        if not skill.embedding or #skill.embedding == 0 then
            goto continue
        end

        local similarity = impressionism.cosine_similarity(message_embedding, skill.embedding)

        -- Deactivate if similarity is below threshold
        if similarity < threshold then
            table.insert(skills_to_deactivate, skill.name)
        end

        ::continue::
    end

    return skills_to_deactivate
end

--- Check if a skill is in the active list.
-- @param name Skill name to check
-- @param active Array of active skill records
-- @return boolean
function is_active(name, active)
    for _, skill in ipairs(active) do
        if skill.name == name then
            return true
        end
    end
    return false
end

return M
