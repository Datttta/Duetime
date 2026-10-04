desktop_file_dir="$HOME/.local/share/applications/"
icon_dir="$HOME/.local/share/icons/hicolor/256x256/apps"
bin_dir="$HOME/.local/bin/"

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
