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

module ttref;

immutable VERSION = "1.1";
string[] g_fileStorages = ["public"];
string[] g_staticStorages = [""];
bool g_enableDistributivePages = false;
bool g_enableGrScript = false;
bool g_enableCompleteProxy = false;
bool g_enable404code = false;
bool g_enableSpawnProcess = false;
string g_grMain = "script/main.gr";
bool g_grhotreload = false;
bool g_grAutoReload = false;