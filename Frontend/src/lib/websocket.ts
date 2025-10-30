/** @format */

export function openRiskWS(onEvent: (e: any) => void) {
     const url = import.meta.env.VITE_WS_URL as string;
     const ws = new WebSocket(url);
     ws.onmessage = (m) => {
          try {
               onEvent(JSON.parse(m.data));
          } catch (error) {
               console.error(
                    "Failed to parse WebSocket message:",
                    error,
                    "Raw data:",
                    m.data,
               );
          }
     };
     return ws;
}
