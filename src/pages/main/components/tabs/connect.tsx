import { useState } from "react"
import { Label } from "../../../../shared/ui/components/ui/label"
import { DatabaseService } from "../../../../services/grpc/databaseService";

export const Connect = () => {

    const [databases, setDatabases] = useState<string[]>([]);

    return (
        <div className="w-full h-full p-2">
            <div className="grid grid-cols-3">
                <Label htmlFor="connectionString" className="self-center" >Connection String:</Label>
                <input type="text" className="col-span-2 focus:outline-none" />
            </div>
            <ul className="flex flex-col">
                {
                    databases.map(db => <li>{db}</li>)
                }
            </ul>
            <button
                className="px-2 py-1 text-sm border bg-teal-500 border-gray-300 hover:border-blue-100"
                onClick={async () => {
                    const databases = await DatabaseService.getDatabases();
                    setDatabases(databases);
                }}
            >Connect</button>
        </div>
    )
}