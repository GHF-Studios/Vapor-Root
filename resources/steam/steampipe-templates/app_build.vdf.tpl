"AppBuild"
{
    "AppID" "{{app_id}}"
    "Desc" "{{description}}"
{{preview_line}}    "SetLive" "{{branch}}"
    "ContentRoot" "{{content_root}}"
    "BuildOutput" "{{build_output}}"
    "Depots"
    {
{{depot_entries}}    }
}
