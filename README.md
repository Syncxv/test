pls help

so basically when i invoke my command from the frontend it says the the command isnt found. when i change the order of patch from last to the first it makes get_config work but the patch no longer works (not found) and when i move the order of patch to the last it makes the patch work but the get_config doesnt work. i have no idea why this is happening and i have tried everything i could think of. any help would be appreciated. thanks in advance



https://github.com/Syncxv/test/assets/47534062/ec93fb22-063c-4a71-975f-356f115553a1



im such a dumbass bro. ITS AN ARRAY IDIOT. you should only call invoke_handler once and pass all the commands in the array FUCK IM SO STUPID MAN WHAT THE HELL https://tauri.app/v1/guides/features/command/#creating-multiple-commands
