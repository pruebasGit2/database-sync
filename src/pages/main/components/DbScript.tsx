type DbScriptProps = {
    database: string,
    scripts: string[]
}

export const DbScript = ({
    database,
    scripts
}: DbScriptProps) => {
    return (
        <div className="w-auto h-6 rounded bg-primary border-foreground hover:border-primary hover:bg-foreground">
            <div className="w-auto h-full text-xs text-foreground hover:text-primary cursor-pointer">
                <div className="w-auto h-full flex items-center pl-1">
                    <span className="mr-2">{database}</span>
                    <div className="h-full aspect-square p-0.5">
                        <div className="w-full h-full rounded-sm bg-foreground text-primary font-semibold flex justify-center items-center">
                            {scripts.length}
                        </div>
                    </div>
                </div>
            </div>
        </div>
    )
}