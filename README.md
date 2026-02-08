# claude-decomment-hook

Remove all useless comments from Claude's generated code.

## Installation

Install the hook into the local environment:

```bash
cargo install --git https://github.com/xorpse/claude-decomment-hook
```

Update Claude's configuration to use the hook:

```json
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Write|Edit|MultiEdit",
        "hooks": [
          {
            "type": "command",
            "command": "claude-decomment-hook"
          }
        ]
      }
    ]
  }
}
```

## Acknowledgement

This code has been ported to Rust from [code-yeongyu/go-claude-code-comment-checker](https://github.com/code-yeongyu/go-claude-code-comment-checker.git).
