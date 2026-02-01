"""Lua module extension for Bazel.

Downloads and builds Lua from source with platform-specific configuration.
"""

# Lua 5.4.7 release (latest stable in 5.4 series)
_LUA_VERSION = "5.4.7"
_LUA_SHA256 = "9fbf5e28ef86c69858f6d3d34eccc32e911c1a28b4120ff3e84aaa70cfbf1e30"
_LUA_URL = "https://www.lua.org/ftp/lua-{version}.tar.gz"

def _lua_repo_impl(rctx):
    """Download and configure Lua source."""
    rctx.download_and_extract(
        url = _LUA_URL.format(version = _LUA_VERSION),
        sha256 = _LUA_SHA256,
        stripPrefix = "lua-" + _LUA_VERSION,
    )

    # Copy our BUILD file into the extracted source
    rctx.file(
        "BUILD.bazel",
        rctx.read(rctx.attr._build_file),
    )

_lua_repo = repository_rule(
    implementation = _lua_repo_impl,
    attrs = {
        "_build_file": attr.label(
            default = "//tools/lua:lua_build.BUILD.bazel",
            allow_single_file = True,
        ),
    },
)

def _lua_impl(mctx):
    """Module extension implementation."""
    _lua_repo(name = "lua")

lua = module_extension(
    implementation = _lua_impl,
)
