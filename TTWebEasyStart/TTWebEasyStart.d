module TTWebEasyStart;

import core.thread.osthread;
import std.process;
void main(string[] args)
{
    import std.process;
    new Thread(&dottweb).start();
    new Thread(&dofrp).start();
}
void dottweb() {
    wait(spawnProcess("./ttweb.exe"));
}
void dofrp() {
    while(true)
    wait(spawnProcess(["./frpc.exe", "-c", "./frpc.ini"]));
}