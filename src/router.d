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

module router;

import serverino;
import ttref;
import htmlautocreate;
import grscript;

import std.file;
import std.experimental.logger;
import std.path;
import std.algorithm;

string xml_isExists(string uri)
{
    if (
        uri[cast(long) uri.length - 1] == '/'
        && (uri ~ "index.html")
        .exists
        && (uri ~ "index.html").isFile
        )
        return uri ~ "/index.html";

    if ((uri ~ ".html").exists && (uri ~ ".html").isFile)
        return uri ~ ".html";

    if ((uri ~ "l").exists && (uri ~ "l").isFile)
        return uri ~ "l";

    if (uri.exists && uri.isFile)
        return uri;

    if ((uri ~ ".xml").exists && (uri ~ ".xml").isFile)
        return uri ~ ".xml";

    return null;
}

@priority(4) @endpoint void mainrouter(Request req, Output output)
{
    if (!g_enableGrScript)
        return;
    grscript_router(req, output);
}

@priority(3) @endpoint void normalpage(Request req, Output output)
{
    if (g_enableGrScript && g_enableCompleteProxy)
        return;

    if (req.uri == "")
        output.write(read("public/index.html"));
    if (g_enableDistributivePages)
    {
        foreach (st; g_fileStorages)
        {
            string pathcache = xml_isExists(st ~ "/" ~ req.uri);
            if (pathcache is null)
                continue;

            output.write(read(pathcache));
        }
    }
    else
    {
        string pathcache = xml_isExists("public" ~ req.uri);
        if (pathcache is null)
            return;

        output.write(read(pathcache));
    }
}

@priority(2) @endpoint void download(const Request req, Output output)
{
    if (g_enableGrScript && g_enableCompleteProxy)
        return;

    foreach (st; g_fileStorages)
    {
        if ((st ~ "/" ~ req.uri).exists && (st ~ "/" ~ req.uri).isFile)
        {
            output.serveFile(st ~ "/" ~ req.uri);
            return;
        }
    }
}

@priority(1) @endpoint void staticStorage(const Request req, Output output)
{

    if (g_enableGrScript && g_enableCompleteProxy)
        return;

    string uri = req.uri;
    if (uri[cast(long) uri.length - 1] == '/')
        uri = uri[0 .. cast(long) uri.length - 1];
    if (uri[0] == '/')
        uri = uri[1 .. cast(long) uri.length];

    foreach (path; g_fileStorages)
    {
        if ((path ~ "/" ~ uri).exists
            && (path ~ "/" ~ uri).isDir
            && find(g_staticStorages, dirName(path ~ "/" ~ uri)))
        {
            try
            {
                output ~= "<a href=\"..\">back</a><br><br><br>" ~ createStaticStorageHTML(uri, path);
                return;
            }
            catch (Exception)
            {
            }

            output ~= "empty";
        }
    }
}

@priority(-1) @endpoint void notfound(const Request req, Output output)
{

    if (g_enableGrScript && g_enableCompleteProxy)
        return;
    if (g_enable404code)
        return;

    log("[USER] " ~ req.uri ~ " Not Found");

    output.status = 404;
    output ~= "public/404.html".read();
    output ~= req.dump();
}

@priority(-16) @endpoint void code_404(Request req, Output output)
{
    if (g_enableGrScript && g_enableCompleteProxy && !g_enable404code)
        return;
    output.status = 404;
}
