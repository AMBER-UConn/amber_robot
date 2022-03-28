
#!/bin/bash
# Make sure to run     chmod a+x install.sh     before running
# To execute  ./install.sh
# Make sure this is run at the root directory
REQUIRED_PACKAGES=( 
    cmake build-essential libudev-dev xorg-dev libglu1-mesa-dev libasound2-dev libxkbcommon-dev # openrr
)

echo "----------------------------- Checking for updates first... -----------------------------"
sudo apt --yes -qq update
echo "----------------------------- Upgrading packages........... -----------------------------"
sudo apt --yes -qq upgrade
echo "----------------------------- Autoremoving old packages.... -----------------------------"
sudo apt -qq autoremove

function checkOrInstall() {
    REQUIRED_PKG=$1
    PKG_OK=$(dpkg-query -W --showformat='${Status}\n' $REQUIRED_PKG|grep "install ok")

    if [ "" = "$PKG_OK" ]; then
        /usr/bin/printf "\xE2\x9D\x8C $REQUIRED_PKG not found. Setting up $REQUIRED_PKG..."
        sudo apt --yes -qq install $REQUIRED_PKG 
    else
        /usr/bin/printf "\xE2\x9C\x94 $REQUIRED_PKG already installed\n"
    fi
}
sudo apt install 

echo "----------------------------- Installing Linux packages -----------------------------"
for PACKAGE in ${REQUIRED_PACKAGES[@]}
do
    checkOrInstall $PACKAGE
done

echo "----------------------------- Installing cargo packages/building codebase -----------------------------"
cargo build

echo "----------------------------- All done! -----------------------------"