import path from "path";
import { loadPackageDefinition, credentials } from '@grpc/grpc-js';
import { loadSync } from '@grpc/proto-loader';

const PROTO_PATH = path.join(__dirname, '../../protos/audio.proto');

console.log(PROTO_PATH)

// Suggested options for similarity to existing grpc.load behavior
const packageDefinition = loadSync(
    PROTO_PATH,
    {
        keepCase: true,
        longs: String,
        enums: String,
        defaults: true,
        oneofs: true
    });

const protoDescriptor = loadPackageDefinition(packageDefinition);
// The protoDescriptor object has the full package hierarchy
const databaseServer = protoDescriptor.database;

//@ts-ignore
const client = new databaseServer.Database('[::1]:777', credentials.createInsecure());

export class DatabaseService {

    static getDatabases(): Promise<string[]>{
        return new Promise((res) => {
            client.getDatabases({}, function name(err: any, databases: {databases: string[]}) {
                if(err){
                    console.log(err)
                    return res([]);
                }
                res(databases.databases);
            })
        });
    } 

}


