import { useSolana } from '@/components/solana/use-solana'
import { WalletDropdown } from '@/components/wallet-dropdown'
import { AppHero } from '@/components/app-hero'
import { PrismpapersdappUiProgramExplorerLink } from './ui/prismpapersdapp-ui-program-explorer-link'
import { PrismpapersdappUiCreate } from './ui/prismpapersdapp-ui-create'
import { PrismpapersdappUiProgram } from '@/features/prismpapersdapp/ui/prismpapersdapp-ui-program'

export default function PrismpapersdappFeature() {
  const { account } = useSolana()

  if (!account) {
    return (
      <div className="max-w-4xl mx-auto">
        <div className="hero py-[64px]">
          <div className="hero-content text-center">
            <WalletDropdown />
          </div>
        </div>
      </div>
    )
  }

  return (
    <div>
      <AppHero title="Prismpapersdapp" subtitle={'Run the program by clicking the "Run program" button.'}>
        <p className="mb-6">
          <PrismpapersdappUiProgramExplorerLink />
        </p>
        <PrismpapersdappUiCreate account={account} />
      </AppHero>
      <PrismpapersdappUiProgram />
    </div>
  )
}
