#!/bin/bash
set -e

desktop_file_dir="$HOME/.local/share/applications/"
icon_dir="$HOME/.local/share/icons/hicolor/256x256/apps"
bin_dir="$HOME/.local/bin/"

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
        mkdir -p $desktop_file_dir $icon_dir $bin_dir

        cat > $desktop_file_dir/Duetime.desktop <<EOF
        [Desktop Entry]
        Type=Application
        Version=1.0
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

        tar -xzvf /tmp/Duetime.tar.gz -C /tmp

        mv /tmp/Duetime $bin_dir

        chmod +x $bin_dir/Duetime

        rm /tmp/Duetime.tar.gz

        echo -e "\nDuetime installed!"
        break
        ;;
    2)
        rm -f $desktop_file_dir/Duetime.desktop
        rm -f $icon_dir/Duetime.png
        rm -f $bin_dir/Duetime

        echo -e "\nDuetime uninstalled."
        break
        ;;
    0)
        exit 0
        ;;
    esac

done

