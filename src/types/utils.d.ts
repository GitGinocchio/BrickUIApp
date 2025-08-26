
declare module "@public/js/utils.js" {
  export function sendResponseRequest(
    port: MessagePort, 
    key: string, 
    type: string, 
    payload: any, 
    timeout: number = 5000
  ): Promise<any>
}