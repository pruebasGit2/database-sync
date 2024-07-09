import { Label } from "../../../../shared/ui/components/ui/label"

export const Connect = () => {
    return (
        <div className="w-full h-full p-2">
            <div className="grid grid-cols-3">
                <Label htmlFor="connectionString" className="self-center" >Connection String:</Label>
                <input type="text" className="col-span-2 focus:outline-none" />
            </div>
            <button className="px-2 py-1 text-sm border bg-teal-500 border-gray-300 hover:border-blue-100">Connect</button>
        </div>
    )
}