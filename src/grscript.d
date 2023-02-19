/* Tiny Tiny Web
 * Copyright (C) 2023 Plasma.
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; 
 * if not, see <https://www.gnu.org/licenses/>.
 */
module grscript;

import serverino;
import grimoire;

import std.experimental.logger;
import std.stdio;
import std.file;
import std.regex;
import ttref;
import std.process;
import std.exception;
import std.datetime;
import std.conv;

GrLibrary ttlib;

GrEngine engine;

Output* routerCache;
Request* reqCache;

string[] stdout_cache;
string uri;

void router_write(GrCall call)
{
    if (call.getString(0).data != "")
    {
        *routerCache ~= call.getString(0).data;
    }
}

void router_serve(GrCall call)
{
    if (call.getString(0).data != "")
    {
        (*routerCache).serveFile(call.getString(0).data);
    }
}

void router_status(GrCall call)
{
    (*routerCache).status = cast(ushort)(call.getInt(0));
}

void read_file(GrCall call)
{
    string f = call.getString(0).data;
    foreach (st; g_fileStorages)
    {
        if ((st ~ f).exists && (st ~ f).isFile)
        {
            call.setString(new GrString(readText(st ~ f)));
            return;
        }
    }

    call.setString(new GrString(""));
}

void write_file(GrCall call)
{
    try
    {
        std.file.write(call.getString(0).data, call.getString(1).data);
        call.setBool(true);
    }
    catch (FileException)
        call.setBool(false);
}

void console_log(GrCall call)
{
    log(cast(LogLevel) call.getInt(0), "[GrRouterLog] " ~ call.getString(1).data);
}

void console_print(GrCall call)
{
    log(LogLevel.all, "[GrRouter] " ~ call.getString(0).data);
}

void regex_(GrCall call)
{
    GrValue[] grstrcache;
    foreach (key; call.getString(0).data.matchAll(call.getString(1).data).front)
    {
        grstrcache ~= GrValue(key);
    }
    call.setList(new GrList(grstrcache));
}

void dump_html(GrCall call)
{
    call.setString(new GrString((*reqCache).dump()));
}

void dump_string(GrCall call)
{
    call.setString(new GrString((*reqCache).dump(false)));
}

void execute_shell(GrCall call)
{
    try
    {
        auto o = executeShell(call.getString(0).data);
        call.setInt(o.status);
        call.setString(o.output);
    }
    catch (StdioException)
    {
        call.setInt(-int.max);
        call.setString(null);
    }
}

void hotreload(GrCall call)
{
    grscript_initialize();
    auto router = grMangleComposite("router", [grString]);
    if (engine.hasEvent(router))
    {
        GrTask task = engine.callEvent(router);
        task.setString("");
    }
    while (engine.hasTasks)
        engine.process();
    if (engine.isPanicking)
        log(LogLevel.fatal, "[GRScript] ERROR: " ~ engine.panicMessage);
}

void get_sec(GrCall call)
{
    call.setInt(Clock.currTime(UTC()).second);
}

void get_min(GrCall call)
{
    call.setInt(Clock.currTime(UTC()).minute);
}

void get_hour(GrCall call)
{
    call.setInt(Clock.currTime(UTC()).hour);
}

void get_day(GrCall call)
{
    call.setInt(Clock.currTime(UTC()).day);
}

void get_month(GrCall call)
{
    call.setInt(Clock.currTime(UTC()).month);
}

void get_year(GrCall call)
{
    call.setInt(Clock.currTime(UTC()).year);
}

void cast_stoi(GrCall call)
{
    try
        call.setInt(call.getString(0).data.to!int);
    catch (Exception)
        call.setInt(-int.max);
}

void ttlibinit()
{
    ttlib = new GrLibrary;
    ttlib.addVariable("TTWEB_VERSION", grString, GrValue(VERSION), true);

    ttlib.addFunction(&router_write, "router_write", [grString]);
    ttlib.addFunction(&router_serve, "router_serve", [grString]);
    ttlib.addFunction(&router_status, "router_status", [grInt]);
    ttlib.addFunction(&read_file, "read_file", [grString], [grString]);
    ttlib.addFunction(&write_file, "write_file", [grString, grString], [grBool]);
    ttlib.addFunction(&console_log, "console_log", [grInt, grString]);
    ttlib.addFunction(&console_print, "console_print", [grString]);
    ttlib.addFunction(&regex_, "regex", [grString, grString], [grList(grString)]); // func (str:string, regex:string)
    ttlib.addFunction(&dump_html, "dump_html", [], [grString]);
    ttlib.addFunction(&dump_string, "dump_string", [], [grString]);
    ttlib.addFunction(&get_sec, "get_sec", [], [grInt]);
    ttlib.addFunction(&get_min, "get_min", [], [grInt]);
    ttlib.addFunction(&get_hour, "get_hour", [], [grInt]);
    ttlib.addFunction(&get_day, "get_day", [], [grInt]);
    ttlib.addFunction(&get_month, "get_month", [], [grInt]);
    ttlib.addFunction(&get_year, "get_year", [], [grInt]);
    ttlib.addCast(&cast_stoi, grString, grInt);
    if (g_grhotreload)
        ttlib.addFunction(&hotreload, "hotreload");
    if (g_enableSpawnProcess)
    {
        ttlib.addFunction(&execute_shell, "execute_shell", [grString], [
                grInt, grString
            ]);

    }
}

void grscript_initialize()
{
    ttlibinit();
    GrLibrary stdlib = grLoadStdLibrary();
    GrCompiler compiler = new GrCompiler;
    compiler.addLibrary(stdlib);
    compiler.addLibrary(ttlib);
    GrBytecode bytecode = compiler.compileFile(g_grMain);
    if (bytecode)
    {
        engine = new GrEngine;

        engine.addLibrary(stdlib);
        engine.addLibrary(ttlib);

        engine.load(bytecode);

        if (engine.hasEvent("start"))
        {
            GrTask task = engine.callEvent("start");
        }
        while (engine.hasTasks)
            engine.process();
        if (engine.isPanicking)
            log(LogLevel.fatal, "[GRScript] ERROR: " ~ engine.panicMessage);

    }
    else
    {
        writeln(compiler.getError().prettify(GrLocale.en_US));
    }

    log("[GRScript] Initialize OK");
}

void grscript_router(Request req, ref Output output)
{

    if (g_grAutoReload)
        grscript_initialize();

    routerCache = &output;
    reqCache = &req;
    uri = req.uri;
    auto router = grMangleComposite("router", [grString]);
    if (engine.hasEvent(router))
    {
        GrTask task = engine.callEvent(router);
        task.setString(req.uri);
    }
    while (engine.hasTasks)
        engine.process();
    if (engine.isPanicking)
        log(LogLevel.fatal, "[GRScript] ERROR: " ~ engine.panicMessage);
    output = *routerCache;
}

void event_init()
{
    if (engine.hasEvent("init"))
    {
        GrTask task = engine.callEvent("init");
    }
    while (engine.hasTasks)
        engine.process();
    if (engine.isPanicking)
        log(LogLevel.fatal, "[GRScript] ERROR: " ~ engine.panicMessage);
}

@onWorkerStop void event_stop()
{
    if (engine.hasEvent("stop"))
    {
        GrTask task = engine.callEvent("stop");
    }
    while (engine.hasTasks)
        engine.process();
    if (engine.isPanicking)
        log(LogLevel.fatal, "[GRScript] ERROR: " ~ engine.panicMessage);
}
