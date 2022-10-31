// Copyright (c) Sandeep Mistry. All rights reserved.
// Licensed under the MIT license. See LICENSE file in the project root for full license information.

// #include <CAN.h> // Uncomment if using arduino IDE
#include "CAN/src/CAN.h" // Uncomment if using vscode
#include "motor.h"

const SMotor motors[] = {
    {.axis = 0, .en_pin = 9, .pin_1 = 2, .pin_2 = 3},
    {.axis = 1, .en_pin = 10, .pin_1 = 5, .pin_2 = 4},
};

typedef enum Commands {
    ERR = 0,
    OK = 1,
    SET_MOTOR = 2
} TREAD_CMD;

void setup()
{
    Serial.begin(9600);
    while (!Serial);

    // Initialize all the pins of the motors
    for (int m = 0; m < sizeof(motors) / sizeof(SMotor); m++) {
        init_motor(&motors[m]);
    }

    Serial.println("CAN Receiver");

    // start the CAN bus at 500 kbps
    if (!CAN.begin(500E3))
    {
        Serial.println("Starting CAN failed!");
        while (1);
    }
}

void loop()
{
    // try to parse packet
    int packetSize = CAN.parsePacket();

    // received a packet
    if (packetSize)
    {
        Serial.print("Received ");

        // RTR packets contain no data
        if (!CAN.packetRtr()) {
            Serial.print(" and length ");
            Serial.println(packetSize);
            
            // Get the tread motor instance
            CAN.packetId()

            // only print packet data for non-RTR packets
            while (CAN.available())
            {
                Serial.print((char)CAN.read());
            }
            Serial.println();
        }
        CAN.packetId()

        Serial.println();
    }
}

// Gets the motor from the array if it is contained
// Returns NULL if not
SMotor * get_motor(int axis) {
    for (int m = 0; m < size; m++) {

        if (motors[m].axis == axis) {
            return (motor_arr + m);
        }
    }
    return NULL;
}


/* The data format for the ID is
<axis ID (4 bits)>  <command ID (6 bits)>
*/
const unsigned int CMD_SIZE = 6;

TREAD_CMD get_packet_cmd(long id) {
    const long mask = 111111;
    int motor_id = id & mask;
}

unsigned int get_axis(long id) {
    const long mask = 1111 << CMD_SIZE;
    int motor_id = (id & mask) >> CMD_SIZE;
}

// Attempts to send CAN command to set the state
// Returns 0 on failure, 1 on success
int set_motor_state(unsigned int axis, MotorDir dir, unsigned int velocity) {
    // Get the particular motor
    for (int i = 0; i < sizeof(TREAD_MOTORS))
}

/*
// Largest axis can be is 2^4 = 16
    if (axis < 0 || axis > 16) {
        CAN.beginPacket(gen_id(axis, ERR));
        CAN.endPacket();
        return 0;
    }
    

    CAN.beginPacket(gen_id(axis, OK));
    int packet_body[8] = {0, 0, 0, 0, 0, 0, (int) dir, velocity};

    CAN.write(packet_body, 8);
    CAN.endPacket();
*/


long gen_id(unsigned int axis, TREAD_CMD cmd) {
    return (axis << CMD_SIZE) | cmd;
}