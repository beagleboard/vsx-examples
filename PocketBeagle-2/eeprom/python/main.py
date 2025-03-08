from beagle_helper import BeagleEeprom
from beagle_helper.boards.pocketbeagle2 import EEPROM_PATH
from pprint import pprint

eeprom_data = BeagleEeprom.read(EEPROM_PATH)
pprint(eeprom_data)
