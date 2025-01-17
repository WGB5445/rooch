// Copyright (c) RoochNetwork
// SPDX-License-Identifier: Apache-2.0

// ** MUI Imports
import Box from '@mui/material/Box'
import IconButton from '@mui/material/IconButton'

// ** Icon Imports
import Icon from 'src/@core/components/icon'

// ** Type Import
import { Settings } from 'src/@core/context/settingsContext'

// ** Components
import LanguageDropdown from 'src/@core/layouts/components/shared-components/LanguageDropdown'
import SwitchChainDropdown from 'src/@core/layouts/components/shared-components/SwitchChainDropdown'
import ModeToggler from 'src/@core/layouts/components/shared-components/ModeToggler'
import UserDropdown from 'src/@core/layouts/components/shared-components/UserDropdown'
import Autocomplete from 'src/layouts/components/Autocomplete'

// ** Hooks
import { useAuth } from 'src/hooks/useAuth'

interface Props {
  hidden: boolean
  settings: Settings
  toggleNavVisibility: () => void
  saveSettings: (values: Settings) => void
}

const AppBarContent = (props: Props) => {
  // ** Props
  const { hidden, settings, saveSettings, toggleNavVisibility } = props

  const auth = useAuth()

  return (
    <Box
      sx={{ width: '100%', display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}
    >
      <Box className="actions-left" sx={{ mr: 2, display: 'flex', alignItems: 'center' }}>
        {hidden ? (
          <IconButton color="inherit" sx={{ ml: -2.75 }} onClick={toggleNavVisibility}>
            <Icon icon="bx:menu" />
          </IconButton>
        ) : null}

        <Autocomplete hidden={hidden} settings={settings} />
      </Box>
      <Box className="actions-right" sx={{ display: 'flex', alignItems: 'center' }}>
        <ModeToggler settings={settings} saveSettings={saveSettings} />
        <LanguageDropdown settings={settings} saveSettings={saveSettings} />
        <SwitchChainDropdown settings={settings} />
        <UserDropdown
          settings={settings}
          data={Array.from(auth.accounts!).map((k, v) => {
            return {
              title: k[1].type,
              address: k[0],
            }
          })}
        />
      </Box>
    </Box>
  )
}

export default AppBarContent
