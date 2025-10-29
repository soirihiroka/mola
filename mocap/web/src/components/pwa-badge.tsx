import type { Component } from 'solid-js'
import { createEffect, Show } from 'solid-js'
import { useRegisterSW } from 'virtual:pwa-register/solid'

import { Dialog } from '@kobalte/core/dialog'
import { showToast } from './toaist'

const PWABadge: Component = () => {
  // check for updates every hour
  const period = 60 * 60 * 1000

  const {
    offlineReady: [offlineReady, setOfflineReady],
    needRefresh: [needRefresh, setNeedRefresh],
    updateServiceWorker,
  } = useRegisterSW({
    onRegisteredSW(swUrl, r) {
      if (period <= 0) return
      if (r?.active?.state === 'activated') {
        registerPeriodicSync(period, swUrl, r)
      }
      else if (r?.installing) {
        r.installing.addEventListener('statechange', (e) => {
          const sw = e.target as ServiceWorker
          if (sw.state === 'activated')
            registerPeriodicSync(period, swUrl, r)
        })
      }
    },
  })

  function close() {
    setOfflineReady(false)
    setNeedRefresh(false)
  }


  createEffect(() => {
    const ready = offlineReady();
    if (ready)
      showToast({ message: "App ready to work offline." })
  })

  return (
    <div class={"card"}
      style={
        { position: "absolute", }
      }
      role="alert" aria-labelledby="toast-message"><div class="begin">
        <Show when={needRefresh()}>
          <Dialog defaultOpen>
            <Dialog.Portal>
              <Dialog.Overlay class="dialog__overlay" />
              <div class="dialog__positioner">
                <Dialog.Content class="dialog__content">
                  <div
                    style={{ display: "flex", "flex-direction": "column", gap: "15px", padding: "30px 25px" }}
                  >
                    <div
                      style={{ "font-weight": 600, "font-size": "16pt", }}
                    >New Version Available</div>
                    <button class='button primary pill' onClick={() => updateServiceWorker(true)} >Update</button>
                    <button class='button pill' onClick={() => close()}>Cancel</button>
                  </div>
                </Dialog.Content>
              </div>
            </Dialog.Portal>
          </Dialog>

        </Show></div>
    </div>
  )
}

export default PWABadge

/**
 * This function will register a periodic sync check every hour, you can modify the interval as needed.
 */
function registerPeriodicSync(period: number, swUrl: string, r: ServiceWorkerRegistration) {
  if (period <= 0) return

  setInterval(async () => {
    if ('onLine' in navigator && !navigator.onLine)
      return

    const resp = await fetch(swUrl, {
      cache: 'no-store',
      headers: {
        'cache': 'no-store',
        'cache-control': 'no-cache',
      },
    })

    if (resp?.status === 200)
      await r.update()
  }, period)
}
