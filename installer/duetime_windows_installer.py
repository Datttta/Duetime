import os
import platform
import subprocess
import shutil
import zipfile
import urllib.request
from pathlib import Path

try:
    import winreg  # type: ignore
except ImportError:
    pass

if platform.system() != "Windows":
    print("This script is for Windows. Use your bash script for Linux.")
    exit(1)

# Ensure pywin32 is installed for shortcut creation
try:
    import win32com.client  # type: ignore
except ImportError:
    print("Installing required package 'pywin32' for shortcuts...")
    subprocess.run(["pip", "install", "pywin32"], check=True)
    import win32com.client  # type: ignore

local_appdata = os.getenv("LOCALAPPDATA")
appdata = os.getenv("APPDATA")

if not local_appdata or not appdata:
    raise RuntimeError("Required environment variables are not set")

# 1. Setup installation directory
dest_folder = Path(local_appdata) / "Duetime"
dest_folder.mkdir(parents=True, exist_ok=True)
exe_path = dest_folder / "Duetime.exe"
icon_path = dest_folder / "Duetime.ico"

print("Downloading latest Duetime release for Windows...")
zip_url = "https://github.com/Datttta/Duetime/releases/latest/download/Duetime-x86_64-pc-windows-gnu.zip"
zip_path = Path("Duetime.zip")

try:
    urllib.request.urlretrieve(zip_url, zip_path)
except Exception as e:
    print(f"Failed to download release: {e}")
    exit(1)

print("Extracting files...")
with zipfile.ZipFile(zip_path, 'r') as zip_ref:
    zip_ref.extractall("extracted_tmp")

# Find Duetime.exe inside the extracted folder structure
extracted_exe_candidates = list(Path("extracted_tmp").rglob("Duetime.exe"))
if not extracted_exe_candidates:
    print("Error: Duetime.exe not found in the downloaded archive.")
    shutil.rmtree("extracted_tmp", ignore_errors=True)
    zip_path.unlink(missing_ok=True)
    exit(1)

extracted_exe = extracted_exe_candidates[0]

# Remove old executable if running
if exe_path.exists():
    try:
        os.remove(exe_path)
    except PermissionError:
        print("Duetime is currently running. Please close the app and re-run the installer.")
        exit(1)

shutil.move(str(extracted_exe), str(exe_path))

# Optional: Download an icon if you publish one to your releases, otherwise fallback to exe
try:
    icon_url = "https://raw.githubusercontent.com/Datttta/Duetime/main/assets/Duetime.ico"
    urllib.request.urlretrieve(icon_url, icon_path)
except Exception:
    icon_path = exe_path  # Fallback to embedding/using the exe icon

# Cleanup temp files
shutil.rmtree("extracted_tmp", ignore_errors=True)
if zip_path.exists():
    zip_path.unlink()

# 2. Create Start Menu Shortcut
start_menu = Path(appdata) / "Microsoft" / "Windows" / "Start Menu" / "Programs"
shortcut_path = start_menu / "Duetime.lnk"

shell = win32com.client.Dispatch("WScript.Shell") # type: ignore
shortcut = shell.CreateShortCut(str(shortcut_path))
shortcut.TargetPath = str(exe_path)
shortcut.WorkingDirectory = str(dest_folder)
shortcut.IconLocation = f"{icon_path},0"
shortcut.save()

# 3. Create an internal uninstall script inside the Duetime folder
uninstall_script_path = dest_folder / "uninstall.py"
uninstall_code = f'''
import os
import shutil
import winreg
from pathlib import Path

# 1. Remove installation folder
dest_folder = Path(r"{dest_folder}")
if dest_folder.exists():
    shutil.rmtree(dest_folder, ignore_errors=True)

# 2. Remove Start Menu shortcut
appdata = os.getenv("APPDATA")
if appdata:
    shortcut_path = Path(appdata) / "Microsoft" / "Windows" / "Start Menu" / "Programs" / "Duetime.lnk"
    if shortcut_path.exists():
        shortcut_path.unlink()

# 3. Remove Registry entry so it disappears cleanly from Add/Remove Programs
try:
    winreg.DeleteKey(winreg.HKEY_CURRENT_USER, r"Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Duetime")
except Exception:
    pass
'''

with open(uninstall_script_path, "w", encoding="utf-8") as f:
    f.write(uninstall_code)

# 4. Register for "Add or Remove Programs" pointing to the uninstall script
uninstall_string = f'pythonw "{uninstall_script_path}"'
uninstall_key_path = r"Software\Microsoft\Windows\CurrentVersion\Uninstall\Duetime"

try:
    with winreg.CreateKey(winreg.HKEY_CURRENT_USER, uninstall_key_path) as key: # type: ignore
        winreg.SetValueEx(key, "DisplayName", 0, winreg.REG_SZ, "Duetime") # type: ignore
        winreg.SetValueEx(key, "Publisher", 0, winreg.REG_SZ, "Datttta") # type: ignore
        winreg.SetValueEx(key, "DisplayIcon", 0, winreg.REG_SZ, str(icon_path)) # type: ignore
        winreg.SetValueEx(key, "UninstallString", 0, winreg.REG_SZ, uninstall_string) # type: ignore
        winreg.SetValueEx(key, "NoModify", 0, winreg.REG_DWORD, 1) # type: ignore
        winreg.SetValueEx(key, "NoRepair", 0, winreg.REG_DWORD, 1) # type: ignore
    print("Registered successfully in Add/Remove Programs.")
except Exception as e:
    print(f"Warning: Could not register in Add/Remove Programs: {e}")

print("Duetime installed!")
