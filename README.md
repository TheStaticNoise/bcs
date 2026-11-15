# bcs (bluetoothctl sucks) - a bluez bluetooth managment util
## scope
Have you ever used bluetoothctl? I doubt that no, so, you also know the pain of using it,
no clear errors, multiple connect commands if the adapter doesn't feel like it, horible
interface, bcs is made to make it way better
## separation
bcs is separated into bcs (the CLI util, with interactive mode) and bcsd (the daemon)
### what does the daemon do?
Daemon manages connections, autoconnects to specified devices, and does much more, 
its a clean separation of responsibilities because making bcs a single monolythic app
would be a terrible idea
### the CLI
The cli connects to the daemon, and allows you to issue commands to it

