import { DownloadTaskEvent } from './bindings.ts'

export type CurrentTabName = 'search' | 'downloaded' | 'comic' | 'reader'

export type ProgressData = Extract<DownloadTaskEvent, { event: 'Create' }>['data'] & {
  percentage: number
  indicator: string
}
