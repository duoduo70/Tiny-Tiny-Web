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
module initialize;

import std.file;
import std.experimental.logger;

import serverino;
import ttref;

immutable ConfigTemplate =
    q{
{
    "Listen": ["127.0.0.1"]
}
};
immutable TTWEB = `
<h1>Tiny Tiny Web `
    ~ VERSION ~ `</h1>

WE HAVE A LICENSE. SEE https://www.gnu.org/licenses/gpl.html.
`;
immutable GrRouter = `
event router(uri: string) {
    router_write(`
    ~ TTWEB ~ `dump_html());
}
`;

@onWorkerStart
void start()
{

    if(!"config.json".exists || !"config.json".isFile) append("config.json", ConfigTemplate);

    if (g_enableGrScript)
    {
        import grscript;

        tryRead("script", { append("script/main.gr", GrRouter); });
        event_init();

    }

    tryRead("public",
    {
        append("public/index.html", TTWEB);
        append("public/404.html", "<h1>Tiny Tiny WEB 404 NOT FOUND</h1>");
    });
    
}

void tryRead(string dirname, void delegate() extrafunc = {})
{
    if (dirname.exists && dirname.isDir)
        return;

    dirname.mkdir;
    extrafunc();
    if (dirname.exists && dirname.isDir)
    {
        log(LogLevel.warning,
            "Can not found dir `" ~ dirname ~ "`. Auto-create is \033[32mSuccessfulled \033[0m");
    }
    else
        log(LogLevel.fatal, "Can not found dir `" ~ dirname ~ "`. Auto-create is \033[31mFailed\033[0m");
}
