#!/bin/bash

HOOK_PATH=".git/hooks/pre-commit"

if [ ! -d ".git" ]; then
    echo "❌ Ce dossier n'est pas un dépôt Git !"
    exit 1
fi

cat << 'EOF' > "$HOOK_PATH"
#!/bin/sh

echo "🔍 Vérification du formatage avec cargo fmt..."

# Exécute cargo fmt en mode check
cargo fmt -- --check
if [ $? -ne 0 ]; then
    echo "❌ Code mal formaté ! Exécutez 'cargo fmt' avant de commit."
    exit 1
fi

echo "✅ Code bien formaté, commit autorisé."
EOF

chmod +x "$HOOK_PATH"

echo "✅ Hook Git 'pre-commit' installé avec succès !"