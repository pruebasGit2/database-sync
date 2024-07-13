import { ReactNode, useMemo, useState } from "react"
import { DatabaseService } from "../../services/grpc/databaseService.tsx";
import { Hash } from "../../interfaces/hash.ts";
import { Db, DbCheck } from "./components/DbCheck.tsx";
import { DbScript } from "./components/DbScript.tsx";
import { open } from "@tauri-apps/api/dialog"
import SyntaxHighlighter from 'react-syntax-highlighter';
import { vs } from "react-syntax-highlighter/dist/esm/styles/hljs";
import { useCommand } from "../../hooks/useCommand.ts";

enum Action {
    DATABASES,
    SCRIPTS,
    SHOW_SCRIPTS
}

export const Main = () => {

    const [isFetching, setIsFetching] = useState(false);

    const [action, setAction] = useState(Action.DATABASES);

    const [cstr, setCstr] = useState("");

    const [databases, setDatabases] = useState<Db[]>([]);

    const [current, setCurrent] = useState(0);
    const [scripts, setScripts] = useState<Hash<string[]>>({});

    const [lastBase, setLastBase] = useState<number | null>(null);
    const [lastChecked, setLastChecked] = useState<number | null>(null);

    const { ref, current: currentCommand } = useCommand({});

    const scriptsArr = useMemo(() => Object.entries(scripts).map(([db, scp]) => ({db, scripts: scp.reduce((p, c) => p + "\n" + c, "")})), [scripts]);

    const changeBase = (index: number, isBase: boolean) => {
        if(currentCommand=="ControlLeft" && lastBase) {
            const from = Math.min(index, lastBase);
            const to = Math.max(index, lastBase);
            setDatabases(db => {
                const _db = [...db];
                const newIsBase = _db[lastBase].isBase;
                for(let i = from;i<=to;i++) {
                    _db[i].isBase = newIsBase;
                }
                return _db;
            });
        } else {
            setDatabases(db => {
                const _db = [...db];
                _db[index].isBase = isBase;
                return _db;
            });
        }

        setLastBase(index);
    }

    const setChecked = (index: number, checked: boolean) => {
        if(currentCommand=="ControlLeft" && lastChecked) {
            const from = Math.min(index, lastChecked);
            const to = Math.max(index, lastChecked);
            setDatabases(db => {
                const _db = [...db];
                const newIsChecked = _db[lastChecked].checked;
                for(let i = from;i<=to;i++) {
                    _db[i].checked = newIsChecked;
                }
                return _db;
            });
        } else {
            setDatabases(db => {
                const _db = [...db];
                _db[index].checked = checked;
                return _db;
            });
        }
        
        setLastChecked(index);
    }

    const GetDatabases = () => {
        setIsFetching(true);
        DatabaseService
            .getDatabases(cstr)
            .then((dbs) => {
                setAction(Action.DATABASES);
                setDatabases(dbs.map((db, i) => ({ id: i, db, checked: false, isBase: false, changeBase, setChecked })));
                setIsFetching(false);
            });
    }

    const Sync = async () => {
        setIsFetching(true);
        setScripts({});

        setAction(Action.SCRIPTS);
        const stream = DatabaseService.getScripts({
            connectionString: cstr,
            databases: databases.filter(db => db.checked).map(db => db.db),
            databasesBase: databases.filter(db => db.isBase).map(db => db.db),
        });
        for await (const res of stream) {
            setScripts(dbs => {
                const _dbs = { ...dbs };
                if (!_dbs[res.database]) _dbs[res.database] = [];
                _dbs[res.database].push(res.script);
                return _dbs;
            });
        }
        setIsFetching(false);
    }

    const save_scripts = async () => {
        const path = await open({
            directory: true,
            multiple: false
        });

        const final: string[] = [];

        Object.values(scripts).forEach(sc => final.push(...sc));

        if (path && typeof path == "string") {
            DatabaseService.saveScripts({
                path,
                scripts: final
            })
        }
    }

    const save_current = async () => {
        const path = await open({
            directory: true,
            multiple: false
        });

        const final: string[] = [];

        Object.values(scripts[scriptsArr[current].db]).forEach(sc => final.push(sc));

        if (path && typeof path == "string") {
            DatabaseService.saveScripts({
                path,
                scripts: final
            })
        }
    }

    return (
        <div ref={ref} className="w-screen h-screen bg-gray-100 pt-[30px] p-2 overflow-hidden" tabIndex={0}>
            <div className="w-full h-full flex flex-col">
                <div className="w-full h-10 flex items-center justify-between text-xs font-normal">
                    <label htmlFor="connectionString" className="flex-1 flex items-center" >Connection String:</label>
                    <input
                        type="text"
                        className="flex-[2] h-6 rounded-sm border border-gray-300 focus:outline-none px-1"
                        value={cstr}
                        onChange={(e) => setCstr(e.target.value)}
                    />
                </div>
                <div className="flex-1 p-2">
                    <div className="p-2 full bg-gray-200 border border-gray-400 rounded-sm">
                        {
                            action == Action.DATABASES ? (
                                <ul className="h-[258px] overflow-y-scroll no-scrollbar">
                                    {databases.map((db, i) => (
                                        <li key={i}>
                                            <DbCheck db={db} changeBase={changeBase} setChecked={setChecked} />
                                        </li>
                                    ))}
                                </ul>
                            ) : action == Action.SCRIPTS ? (
                                <ul className="h-[258px] flex flex-wrap gap-2 overflow-y-scroll no-scrollbar wrap">
                                    {Object.entries(scripts).map(([db, scripts], i) => (
                                        <li key={i} className="py-4 h-5 flex justify-center items-center">
                                            <DbScript
                                                database={db}
                                                scripts={scripts}
                                                onClick={() => {
                                                    setCurrent(i);
                                                    setAction(Action.SHOW_SCRIPTS);
                                                }}
                                            />
                                        </li>
                                    ))}
                                </ul>
                            ) : (
                                <div className="h-[258px] relative w-full overflow-y-scroll no-scrollbar flex flex-col">
                                    <SyntaxHighlighter
                                        customStyle={{fontSize: "0.65rem", lineHeight: "0.75rem", flex: 1}}
                                        language="sql"
                                        style={vs}
                                    >
                                        {scriptsArr[current].scripts}
                                    </SyntaxHighlighter>
                                    <div className="w-full h-6 flex justify-between">
                                        <Button
                                            disabled={current == 0}
                                            onClick={() => setCurrent(c => Math.max(0, c-1))}
                                        >
                                            Back
                                        </Button>
                                        <Button onClick={save_current}>Save: [{scriptsArr[current].db}]</Button>
                                        <Button
                                            disabled={current == scriptsArr.length-1}
                                            onClick={() => setCurrent(c => Math.min(scriptsArr.length, c+1))}
                                        >
                                            Next
                                        </Button>
                                    </div>
                                </div>
                            )
                        }
                    </div>
                </div>
                <div className="h-10">
                    <hr className="border border-gray-300" />
                    <div className="w-full h-full flex items-center justify-end gap-2">
                        {
                            (Object.keys(scripts).length > 0) && (
                                <>
                                    {
                                        (action == Action.SCRIPTS) 
                                            ? <Button onClick={() => setAction(Action.SHOW_SCRIPTS)}>Show scripts</Button> 
                                            : <Button onClick={() => setAction(Action.SCRIPTS)}>Show databases</Button>
                                    }
                                    <Button onClick={save_scripts}>Save all scripts</Button>
                                </>
                            )
                        }
                        {
                            (databases.length > 0) && <Button disabled={isFetching} onClick={Sync}>Sync</Button>
                        }
                        {
                            (action != Action.DATABASES) ? (
                                <Button onClick={() => setAction(Action.DATABASES)}>Select databases</Button>
                            ) : (
                                <Button disabled={isFetching} onClick={GetDatabases}>Get databases</Button>
                            )
                        }
                    </div>
                </div>
            </div>
        </div>
    )
}

type ButtonProps = {
    disabled?: boolean,
    onClick?: React.MouseEventHandler<HTMLButtonElement>,
    children?: ReactNode
}

const Button = ({ onClick, disabled, children }: ButtonProps) => {
    return (
        <button
            disabled={disabled}
            className="h-6 px-2 text-sm rounded-md border bg-primary text-foreground border-foreground hover:border-primary hover:bg-foreground hover:text-primary disabled:cursor-not-allowed"
            onClick={onClick}
        >
            {children}
        </button>
    )
}