import { useEffect, useMemo, useRef, useState } from "react"

export type CommandData = {
    id: string,
    name: string,
    command: string,
    description?: string,
    group?: string,
    handle?: (command: CommandData) => void
}

type useCommandType = (props: {
    commands?: CommandData[]
}) => {
    ref: React.RefObject<HTMLDivElement>,
    event: CommandData | null,
    current: string | null,
    last: string | null
}

export const useCommand: useCommandType = ({
    commands: default_commands
}) => {

    const ref = useRef<HTMLDivElement>(null);
    
    const commands: { [key: string]: CommandData } = useMemo(() => {
        const lookup: { [key: string]: CommandData } = {};
        if (default_commands) {
            default_commands.forEach(c => {
                lookup[c.command] = c;
            });
        }
        return lookup;
    }, [default_commands]);

    const [event, setEvent] = useState<CommandData | null>(null);
    const [current, setCurrent] = useState<string | null>(null);
    const [keysPressed, setKeysPressed] = useState<string[]>([]);
    const [maxPressed, setMaxPressed] = useState<string[]>([]);

    useEffect(() => {
        if (!ref.current) return;

        const HandleKeyDown = (ev: KeyboardEvent) => {
            setKeysPressed(k => [...k.filter(k => k !== ev.code), ev.code]);
        }
        const HandleKeyUp = (ev: KeyboardEvent) => {
            setKeysPressed(k => [...k.filter(k => k !== ev.code)]);
        }

        ref.current.addEventListener("keydown", HandleKeyDown);
        ref.current.addEventListener("keyup", HandleKeyUp);
        return () => {
            ref.current?.removeEventListener("keydown", HandleKeyDown);
            ref.current?.removeEventListener("keyup", HandleKeyUp);
        }
    }, []);

    useEffect(() => {
        if(keysPressed.length == 0) setMaxPressed([]);
        setMaxPressed(m => {
            if(m.length > keysPressed.length) return m;
            else return [...keysPressed];
        });
    }, [keysPressed]);

    const last = useMemo(() => {
        return maxPressed.join(":")
    }, [maxPressed]);

    useEffect(() => {
        if (keysPressed.length == 0) {
            setCurrent("");
            return;
        }

        const current = keysPressed.join(":");
        
        setCurrent(current);

        const command = commands[current];

        setEvent(command ? {...command} : null);
        setCurrent(current);
        if(command?.handle) command.handle(command);

    }, [keysPressed, commands]);

    return {
        ref,
        event,
        current,
        last
    };
}

