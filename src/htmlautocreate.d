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

module htmlautocreate;

import ttref;
import std.file;
import std.path;
import std.experimental.logger;

string createStaticStorageHTML(string dir, string path)
{
    string eles = "";
    string fullpath = path ~ "/" ~ dir;
    foreach (key; fullpath.dirEntries(SpanMode.shallow))
    {
        eles ~= "<a href=\"/" ~ dir ~ "/" ~ baseName(key) ~ "\">" ~ baseName(key) ~ "</a><br>";
    }

    return eles;
}
