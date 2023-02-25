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
module config;

import std.stdio;
import std.file;
import std.json;
import std.experimental.logger;
import std.algorithm;
import std.conv;
import core.time;

import serverino;
import ttref;

auto readjson()
{
    if ("config.json".exists && "config.json".isFile)
        return parseJSON(cast(string) "config.json".read());
    else
        return parseJSON(`{ "Listen": ["127.0.0.1"]

}`);
}

@onServerInit ServerinoConfig setup()
{
    ServerinoConfig sc = ServerinoConfig.create();
    JSONValue json;

    try
    {
        json = readjson();
        json["Listen"].array;
    }
    catch (std.json.JSONException)
    {
        log(LogLevel.fatal, "Config Error!");
    }

    foreach (key; json["Listen"].array)
    {
        string k_str = key.str;
        string IPType = "IPv4";
        string IP = "";
        string Port = "";

        bool flag_ipv6_end = false;
        bool flag_port_get_ok = false;
        for (ulong i = cast(long) k_str.length - 1; i >= 0; i--)
        {
            if (i == 0 && flag_ipv6_end)
            {
                IPType = "IPv6";
                if (Port.length == 0)
                    Port = "80";
                break;
            }
            if (i == 0)
            {
                if (Port.length == 0)
                    Port = "80";
                break;
            }
            if (k_str[i] == ']')
                flag_ipv6_end = true;
            if (k_str[i] == ':' && !flag_port_get_ok && !flag_ipv6_end)
            {
                for (ulong j = i + 1; j < k_str.length; j++)
                {
                    Port ~= k_str[j];
                }
                flag_port_get_ok = true;
            }
        }
        for (ulong i = 0; i < k_str.length; i++)
        {
            if (k_str[i] == '[')
                continue;
            if (i == k_str.length)
            {
                IP = "127.0.0.1";
                break;
            }
            if (k_str[i] == ']')
                break;
            if (IPType == "IPv4" && k_str[i] == ':')
                break;
            IP ~= k_str[i];
        }

        if (IPType == "IPv6")
            sc.addListener!(ServerinoConfig.ListenerProtocol.IPV6)(IP, cast(ushort) Port.to!int);
        else
            sc.addListener(IP, cast(ushort) Port.to!int);

    }

    trysetting({ sc.setReturnCode(json["ReturnCode"].get!int); }, "ReturnCode");
    try
    {
        json["Workers"];
    }
    catch (std.json.JSONException)
    {
        trysetting({ sc.setMaxWorkers(json["MaxWorkers"].get!size_t); }, "MaxWorkers");
        trysetting({ sc.setMinWorkers(json["MinWorkers"].get!size_t); }, "MinWorkers");
    }
    trysetting({ sc.setWorkers(json["Workers"].get!size_t); }, "Workers");
    trysetting({
        sc.setMaxWorkerLifetime((json["MaxWorkerLifetime"].get!int).dur!"hours");
    }, "MaxWorkerLifetime");
    trysetting({
        sc.setMaxWorkerIdling((json["MaxWorkerIdling"].get!int).dur!"hours");
    }, "MaxWorkerIdling");
    trysetting({
        sc.setMaxRequestTime((json["MaxRequestTime"].get!int).dur!"seconds");
    }, "MaxRequestTime");
    trysetting({
        sc.setHttpTimeout((json["HttpTimeout"].get!int).dur!"seconds");
    }, "HttpTimeout");
    trysetting({ sc.setListenerBacklog(json["ListenerBacklog"].get!int); }, "ListenerBacklog");
    trysetting({ sc.setMaxRequestSize(json["MaxRequestSize"].get!size_t); }, "MaxRequestSize");
    version (Windows)
    {
    }
    else
    {
        trysetting({ sc.setWorkerUser(json["WorkerUser"].get!string); }, "WorkerUser");
        trysetting({ sc.setWorkerGroup(json["WorkerGroup"].get!string); }, "WorkerGroup");
    }

    Duration keepAliveTimeout = 3.dur!"seconds";
    try
        keepAliveTimeout = (json["KeepAliveTimeout"].get!int).dur!"seconds";
    catch (std.json.JSONException)
    {
    }
    trysetting({
        sc.enableKeepAlive(json["KeepAlive"].get!bool, keepAliveTimeout);
    }, "KeepAlive");

    try
    {
        foreach (key; json["ExtraFileStorages"].array)
        {
            string k = key.str;
            if (k.exists && k.isDir)
            {
                g_fileStorages ~= k;
            }
            else
                log(LogLevel.error, "ExtraFileStorages: \"" ~ k ~ "\" is not a dir");
        }
    }
    catch (std.json.JSONException)
    {
    }

    try
    {
        if (json["EnableDistributivePage"].get!bool)
        {
            g_enableDistributivePages = true;
        }
    }
    catch (std.json.JSONException)
    {
    }

    try
    {
        foreach (key; json["StaticStorages"].array)
        {
            string k = key.str;
            if (k.exists && k.isDir)
            {
                g_staticStorages ~= k;
            }
            else
                log(LogLevel.error, "staticStorages: \"" ~ k ~ "\" is not a dir");
        }
    }
    catch (std.json.JSONException)
    {
    }

    try
    {
        if (json["ProxyRouter"].get!bool)
        {
            g_enableGrScript = true;
            try
            {
                if (json["CompleteProxy"].get!bool)
                {
                    g_enableCompleteProxy = true;
                }
            }
            catch (std.json.JSONException)
            {
            }
        }
    }
    catch (std.json.JSONException)
    {
    }

    try
    {
        if (json["NotFoundForCode"].get!bool)
        {
            g_enable404code = true;
        }
    }
    catch (std.json.JSONException)
    {
    }

    try
    {
        if (json["EnableSpawnProcess"].get!bool)
        {
            g_enableSpawnProcess = true;
        }
    }
    catch (std.json.JSONException)
    {
    }

    try
    {
        g_grMain = json["GrMain"].get!string;
    }
    catch (std.json.JSONException)
    {
    }

    try
    {
        g_grhotreload = json["GrHotReload"].get!bool;
    }
    catch (std.json.JSONException)
    {
    }

    try
    {
        g_grAutoReload = json["GrAutoReload"].get!bool;
    }
    catch (std.json.JSONException)
    {
    }

    if(g_enableGrScript) {
        import grscript;
        grscript_initialize();
    }

    return sc;
}

void trysetting(void delegate() scfunc, string settingName, LogLevel lv = LogLevel.info)
{
    try
    {
        scfunc();
    }
    catch (std.json.JSONException)
    {
        log(lv, "Can not found " ~ settingName ~ " in config.json. Used default setting");
    }
}
