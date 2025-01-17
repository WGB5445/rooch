// Copyright (c) RoochNetwork
// SPDX-License-Identifier: Apache-2.0

// ** Rooch SDK
import { IAccount } from '@rooch/sdk'

export interface Session {
  account: IAccount | null
  loading: boolean
  errorMsg: string | null
  requestAuthorize?: (scope: Array<string>, maxInactiveInterval: number) => Promise<void>
  close: () => void
}
