module build;
import std.conv;
import std.stdio;
import std.process;
import std.regex;
import std.file;
import std.algorithm;

static import core.exception;

immutable srcDir = "src";

version(Windows) {
immutable exeName = "ttweb.exe";
} else {
immutable exeName = "ttweb";
}

immutable string[] defaultargs =
    [
        "--od=temp",
        "--extern-std=c++20",
        "--of=" ~ exeName
    ];

immutable doc =
    `USAGE: build.exe ldc2(DEFAULT)|ldmd2|rdmd [EXTRA_OPTIONS_FOR_COMMPILER]
`;

struct BuildArgs
{
    string compiler;
    string[] supplementaryArgs;
}

void main(string[] argv)
{
    auto args = new BuildArgs;

    if (argv.length >= 2 && (argv[1] == "-h" || argv[1] == "--help"))
    {
        writeln(doc);
        return;
    }

    try
    {
        args.compiler = matchFirst(argv[1], "ldc2||ldmd2||rdmd2").front;
        if (args.compiler == null)
        {
            args.compiler = "ldc2";
            for (int i = 1; i < argv.length; i++)
        {
            args.supplementaryArgs ~= argv[i];
        }
        } else for (int i = 2; i < argv.length; i++)
        {
            args.supplementaryArgs ~= argv[i];
        }
    }
    catch (core.exception.ArrayIndexError e)
    {
        args.compiler = "ldc2";
        args.supplementaryArgs = [];
    }

    string[] srcfiles;

    foreach (string name; dirEntries(srcDir, SpanMode.depth).filter!(f => f.name.endsWith(".d")))
    {
        srcfiles ~= name;
    }

    auto pid = spawnProcess([args.compiler] ~ defaultargs ~ args.supplementaryArgs ~ srcfiles);
    int returnValue = wait(pid);
    if (returnValue != 0)
    {
        writeln("\033[31mBuild Failed\033[0m (code: "~returnValue.to!string~")");
        return;
    }

    writeln("\033[32mBuild Succed\033[0m");

    writeln();

    pid = spawnProcess("./" ~ exeName);
    returnValue = wait(pid);
    if (returnValue != 0)
    {
        writeln("\033[31mRun Failed\033[0m (code: "~returnValue.to!string~")");
        return;
    }

}
