#!/bin/bash

if [ -f /etc/rc.local ]; then 
    echo "rc.local exists!"
    
else
    printf '%s\n' '#!/bin/bash' '' 'exit 0' | sudo tee -a /etc/rc.local

    # Make the rc.local file executable
    sudo chmod +x /etc/rc.local
fi 


# If the rc.local file does not have the can_setup script line, add it 
STRING="sh /can_setup.sh &"
FILE=/etc/rc.local

if ! grep -q "$STRING" "$FILE"; then
    echo "can_setup.sh not included in rc.local... adding"
    # Insert the command to run the script in the second line
    LINE_NUMBER=2
    sudo sed -i -e "${LINE_NUMBER}i\\" -e "$STRING" /etc/rc.local
fi

exit 0
