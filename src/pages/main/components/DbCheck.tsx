import { Check } from "lucide-react"

type DbCheckProps = { db: Db }

export type Db = {
    id: number,
    db: string,
    checked: boolean,
    isBase: boolean,
    setChecked: (index: number, checked: boolean) => void,
    changeBase: (index: number, isBase: boolean) => void
}

export const DbCheck = ({ db }: DbCheckProps) => {

    return (
        <div
            className="w-full h-6 rounded border border-gray-200 bg-gray-100 text-sm flex items-center justify-between px-2"
            onClick={() => {
                db.setChecked(db.id, true);
                db.changeBase(db.id, !db.isBase);
            }}
        >
            <div className="flex gap-2">
                {
                    db.isBase
                        ? <Check className="h-4 w-4" />
                        : <div className="h-4 w-4" />
                }
                <input
                    type="checkbox"
                    className="cursor-pointer"
                    checked={db.checked}
                    onChange={() => {
                        if (db.isBase) return;
                        db.setChecked(db.id, !db.checked);
                    }}
                    onClick={(e) => e.stopPropagation()}
                />
            </div>
            {db.db}
        </div>
    )
}