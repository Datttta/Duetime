#!/bin/bash
set -e

desktop_file_dir="$HOME/.local/share/applications"
icon_dir="$HOME/.local/share/icons/hicolor/256x256/apps"
user_bin_dir="$HOME/.local/bin"
link_bin_dir="/usr/local/bin"

while true; do
    echo "
================= Installation hub ===================
1 - Install
2 - Uninstall
0 - Exit
======================================================
    "

    read -p "Choice: " choice

    case $choice in
    1)
        mkdir -p "$desktop_file_dir" "$icon_dir" "$user_bin_dir"

        cat > "$desktop_file_dir/Duetime.desktop" <<EOF
[Desktop Entry]
Type=Application
Name=Duetime
Comment=A terminal task and time management application
Exec=Duetime
Icon=Duetime
Terminal=true
Categories=Utility;ConsoleOnly;
EOF

        curl -fL \
            "https://raw.githubusercontent.com/Datttta/Duetime/main/assets/Duetime.png" \
            -o "$icon_dir/Duetime.png"

        curl -fL \
            "https://github.com/Datttta/Duetime/releases/latest/download/Duetime-x86_64-unknown-linux-gnu.tar.gz" \
            -o /tmp/Duetime.tar.gz

        tar -xzf /tmp/Duetime.tar.gz -C /tmp

        install -Dm755 /tmp/Duetime "$user_bin_dir/Duetime"
        sudo ln -sf "$user_bin_dir/Duetime" "$link_bin_dir/Duetime"
        sudo rm -f /usr/bin/Duetime /bin/Duetime

        rm -f /tmp/Duetime /tmp/Duetime.tar.gz
        update-desktop-database "$desktop_file_dir" 2>/dev/null || true

        echo -e "\nDuetime installed!"
        break
        ;;
    2)
        rm -f "$desktop_file_dir/Duetime.desktop"
        rm -f "$icon_dir/Duetime.png"
        rm -f "$user_bin_dir/Duetime"
        sudo rm -f "$link_bin_dir/Duetime" /usr/bin/Duetime /bin/Duetime
        update-desktop-database "$desktop_file_dir" 2>/dev/null || true

        echo -e "\nDuetime uninstalled."
        break
        ;;
    0)
        exit 0
        ;;
    esac

done
