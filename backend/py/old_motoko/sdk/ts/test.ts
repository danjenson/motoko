import { grpc } from '@improbable-eng/grpc-web'
import { MotokoClient } from './motoko_pb_service'
import { InferRequest, InferResponse } from './types_pb'
import * as WebSocket from 'ws'

grpc.setDefaultTransport(grpc.WebsocketTransport())
const motoko = new MotokoClient('https://localhost')
const infer = motoko.webInfer()
const process = (chunk: Uint8Array): void => {
  const req = new InferRequest()
  req.setData(chunk)
  infer.write(req)
}
const progress = (p: number): void => console.log(p * 100)
const end = (): void => infer.end()
infer.on('data', function(res: InferResponse) {
  console.log(res.getMetadata())
})
const file = new File(["a,b\na,2\nb,2"], "/tmp/test.csv")
stream(file, process, progress, end)


function stream(
  file: File,
  process: (chunk: Uint8Array) => void,
  progress: (proportion: number) => void,
  end: () => void,
  chunkSize = 1e6,
): void {
  let offset = 0;

  function handleChunk(event: ProgressEvent): void {
    const reader = event.target as FileReader
    const contents: Uint8Array = new Uint8Array(reader.result as ArrayBuffer)
    if (contents) {
      offset += contents.length
      process(contents)
      progress(offset / file.size)
    }
    if (offset === file.size) {
      end()
      return
    }
    readChunk()
  }

  function readChunk(): void {
    const reader = new FileReader()
    const blob = file.slice(offset, offset + chunkSize)
    reader.onload = handleChunk
    reader.onerror = (e): void => console.log(e)
    reader.readAsArrayBuffer(blob)
  }

  readChunk()
}
