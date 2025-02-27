from beagle_linux_sdk import BeagleEeprom
from beagle_linux_sdk.board.pocketbeagle2 import EEPROM_PATH
from pprint import pprint

eeprom_data = BeagleEeprom.read(EEPROM_PATH)
pprint(eeprom_data)
