import { PRISMPAPERSDAPP_PROGRAM_ADDRESS } from '@project/anchor'
import { AppExplorerLink } from '@/components/app-explorer-link'
import { ellipsify } from '@wallet-ui/react'

export function PrismpapersdappUiProgramExplorerLink() {
  return <AppExplorerLink address={PRISMPAPERSDAPP_PROGRAM_ADDRESS} label={ellipsify(PRISMPAPERSDAPP_PROGRAM_ADDRESS)} />
}
