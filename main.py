import serial
import re

ser = serial.Serial(port='COM3', baudrate=57600)


def main():
    ser.reset_output_buffer()
    ser.reset_input_buffer()
    shouldRun = True
    operation = ''

    while shouldRun:
        operation = input("Entrez une opération (0-9, p, c, q):")

        if len(operation) == 1:

            if operation == 'p':
                ser.write((11).to_bytes(1, byteorder='big'))
            elif operation == 'c':
                ser.write((12).to_bytes(1, byteorder='big'))
            elif re.search('^[0-9]$', operation):
                ser.write(int(operation).to_bytes(1, byteorder='big'))
            elif operation == 'q':
                shouldRun = False


if __name__ == '__main__':
    main()
