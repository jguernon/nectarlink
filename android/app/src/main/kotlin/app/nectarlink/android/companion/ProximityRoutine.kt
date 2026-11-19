// SPDX-License-Identifier: GPL-3.0-or-later
package app.nectarlink.android.companion

import android.content.Context
import androidx.core.content.edit
import app.nectarlink.android.NectarlinkApplication

/**
 * The proximity routine: with it on, the phone stays connected to the chosen
 * PC only while the PC's Bluetooth beacon is in range. When Android's
 * Companion Device Manager reports the beacon gone, the PC is disconnected
 * ("Stop reconnecting"); when the beacon is back, it reconnects.
 *
 * Only a disconnect the routine made is undone by it: a PC the user
 * disconnected by hand stays disconnected.
 */
object ProximityRoutine {
    private const val PREFS = "appearance"
    private const val ENABLED = "proximity_routine"
    private const val PC_ID = "proximity_routine_pc"
    private const val PAUSED = "proximity_routine_paused"

    private fun prefs(context: Context) =
        context.applicationContext.getSharedPreferences(PREFS, Context.MODE_PRIVATE)

    fun isEnabled(context: Context): Boolean = prefs(context).getBoolean(ENABLED, false)

    fun pcId(context: Context): String? = prefs(context).getString(PC_ID, null)

    /** Turns the routine on for [pcId], or off (reconnecting a PC it had disconnected). */
    fun setEnabled(context: Context, enabled: Boolean, pcId: String?) {
        val wasPaused = prefs(context).getBoolean(PAUSED, false)
        val previousPc = pcId(context)
        prefs(context).edit {
            putBoolean(ENABLED, enabled && pcId != null)
            if (pcId != null) putString(PC_ID, pcId)
            if (!enabled) putBoolean(PAUSED, false)
        }
        if (!enabled && wasPaused && previousPc != null) {
            core(context)?.setReconnect(previousPc, true)
        }
    }

    /** The PC's beacon is in range: reconnect it if the routine disconnected it. */
    fun onPcNear(context: Context) {
        if (!isEnabled(context)) return
        val pc = pcId(context) ?: return
        if (!prefs(context).getBoolean(PAUSED, false)) return
        val core = core(context) ?: return
        prefs(context).edit { putBoolean(PAUSED, false) }
        core.start()
        core.setReconnect(pc, true)
    }

    /** The PC's beacon is out of range: disconnect it, unless the user already did. */
    fun onPcAway(context: Context) {
        if (!isEnabled(context)) return
        val pc = pcId(context) ?: return
        val core = core(context) ?: return
        val device = core.state.value.devices.firstOrNull { it.id == pc }
        if (device != null && !device.reconnect) return
        prefs(context).edit { putBoolean(PAUSED, true) }
        core.start()
        core.setReconnect(pc, false)
    }

    private fun core(context: Context) = (context.applicationContext as? NectarlinkApplication)?.core
}
