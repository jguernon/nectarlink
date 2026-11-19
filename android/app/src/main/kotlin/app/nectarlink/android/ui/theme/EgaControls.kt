// SPDX-License-Identifier: GPL-3.0-or-later
package app.nectarlink.android.ui.theme

import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.interaction.MutableInteractionSource
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.RowScope
import androidx.compose.foundation.layout.offset
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.selection.toggleable
import androidx.compose.foundation.shape.CornerSize
import androidx.compose.material3.ButtonColors
import androidx.compose.material3.ButtonDefaults
import androidx.compose.material3.ButtonElevation
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.SwitchColors
import androidx.compose.material3.SwitchDefaults
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.RectangleShape
import androidx.compose.ui.graphics.Shape
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp

/*
 * The EGA theme draws everything square. Material's buttons, switches and the
 * round shapes written in the screens don't follow the theme's Shapes, so the
 * screens import these instead of Material's (same names, same parameters):
 * in EGA they are square, otherwise they are exactly Material's.
 */

/** Square in the EGA theme, else [shape]. Read in composition (it follows a theme switch). */
private fun egaOr(shape: Shape): Shape = if (egaSquareCorners) RectangleShape else shape

// ---- Shapes ----

/** [androidx.compose.foundation.shape.RoundedCornerShape], square in EGA. */
fun RoundedCornerShape(size: Dp): Shape =
    egaOr(androidx.compose.foundation.shape.RoundedCornerShape(size))

fun RoundedCornerShape(percent: Int): Shape =
    egaOr(androidx.compose.foundation.shape.RoundedCornerShape(percent))

fun RoundedCornerShape(size: CornerSize): Shape =
    egaOr(androidx.compose.foundation.shape.RoundedCornerShape(size))

fun RoundedCornerShape(
    topStart: Dp = 0.dp,
    topEnd: Dp = 0.dp,
    bottomEnd: Dp = 0.dp,
    bottomStart: Dp = 0.dp,
): Shape = egaOr(androidx.compose.foundation.shape.RoundedCornerShape(topStart, topEnd, bottomEnd, bottomStart))

fun RoundedCornerShape(
    topStartPercent: Int = 0,
    topEndPercent: Int = 0,
    bottomEndPercent: Int = 0,
    bottomStartPercent: Int = 0,
): Shape = egaOr(
    androidx.compose.foundation.shape.RoundedCornerShape(topStartPercent, topEndPercent, bottomEndPercent, bottomStartPercent),
)

/** [androidx.compose.foundation.shape.CircleShape], square in EGA. */
val CircleShape: Shape
    get() = egaOr(androidx.compose.foundation.shape.CircleShape)

// ---- Buttons ----

@Composable
fun Button(
    onClick: () -> Unit,
    modifier: Modifier = Modifier,
    enabled: Boolean = true,
    shape: Shape = ButtonDefaults.shape,
    colors: ButtonColors = ButtonDefaults.buttonColors(),
    elevation: ButtonElevation? = ButtonDefaults.buttonElevation(),
    border: BorderStroke? = null,
    contentPadding: PaddingValues = ButtonDefaults.ContentPadding,
    interactionSource: MutableInteractionSource? = null,
    content: @Composable RowScope.() -> Unit,
) = androidx.compose.material3.Button(
    onClick, modifier, enabled, egaOr(shape), colors, if (egaSquareCorners) null else elevation,
    border, contentPadding, interactionSource, content,
)

@Composable
fun FilledTonalButton(
    onClick: () -> Unit,
    modifier: Modifier = Modifier,
    enabled: Boolean = true,
    shape: Shape = ButtonDefaults.filledTonalShape,
    colors: ButtonColors = ButtonDefaults.filledTonalButtonColors(),
    elevation: ButtonElevation? = ButtonDefaults.filledTonalButtonElevation(),
    border: BorderStroke? = null,
    contentPadding: PaddingValues = ButtonDefaults.ContentPadding,
    interactionSource: MutableInteractionSource? = null,
    content: @Composable RowScope.() -> Unit,
) = androidx.compose.material3.FilledTonalButton(
    onClick, modifier, enabled, egaOr(shape), colors, if (egaSquareCorners) null else elevation,
    border, contentPadding, interactionSource, content,
)

@Composable
fun OutlinedButton(
    onClick: () -> Unit,
    modifier: Modifier = Modifier,
    enabled: Boolean = true,
    shape: Shape = ButtonDefaults.outlinedShape,
    colors: ButtonColors = ButtonDefaults.outlinedButtonColors(),
    elevation: ButtonElevation? = null,
    border: BorderStroke? = ButtonDefaults.outlinedButtonBorder(enabled),
    contentPadding: PaddingValues = ButtonDefaults.ContentPadding,
    interactionSource: MutableInteractionSource? = null,
    content: @Composable RowScope.() -> Unit,
) = androidx.compose.material3.OutlinedButton(
    onClick, modifier, enabled, egaOr(shape), colors, elevation, border, contentPadding, interactionSource, content,
)

@Composable
fun TextButton(
    onClick: () -> Unit,
    modifier: Modifier = Modifier,
    enabled: Boolean = true,
    shape: Shape = ButtonDefaults.textShape,
    colors: ButtonColors = ButtonDefaults.textButtonColors(),
    elevation: ButtonElevation? = null,
    border: BorderStroke? = null,
    contentPadding: PaddingValues = ButtonDefaults.TextButtonContentPadding,
    interactionSource: MutableInteractionSource? = null,
    content: @Composable RowScope.() -> Unit,
) = androidx.compose.material3.TextButton(
    onClick, modifier, enabled, egaOr(shape), colors, elevation, border, contentPadding, interactionSource, content,
)

// ---- Switch ----

/**
 * Material's switch, or in EGA a DOS-style toggle: a square box with a square
 * block that sits left (off, gray) or right (on, yellow on blue).
 */
@Composable
fun Switch(
    checked: Boolean,
    onCheckedChange: ((Boolean) -> Unit)?,
    modifier: Modifier = Modifier,
    thumbContent: (@Composable () -> Unit)? = null,
    enabled: Boolean = true,
    colors: SwitchColors = SwitchDefaults.colors(),
    interactionSource: MutableInteractionSource? = null,
) {
    if (!egaSquareCorners) {
        androidx.compose.material3.Switch(checked, onCheckedChange, modifier, thumbContent, enabled, colors, interactionSource)
        return
    }
    val scheme = MaterialTheme.colorScheme
    val dim = if (enabled) 1f else 0.38f
    val frame = (if (checked) scheme.outline else scheme.outlineVariant).copy(alpha = dim)
    val track = if (checked) scheme.primaryContainer else Color.Transparent
    val block = (if (checked) scheme.primary else scheme.onSurfaceVariant).copy(alpha = dim)
    val toggle = if (onCheckedChange != null) {
        Modifier.toggleable(
            value = checked,
            enabled = enabled,
            role = Role.Switch,
            onValueChange = onCheckedChange,
        )
    } else {
        Modifier
    }
    // Same footprint as Material's switch (52 × 32 dp), so rows keep their layout.
    Box(modifier.then(toggle).size(52.dp, 32.dp), contentAlignment = Alignment.CenterStart) {
        Box(
            Modifier
                .padding(horizontal = 2.dp, vertical = 4.dp)
                .size(48.dp, 24.dp)
                .background(track)
                .border(2.dp, frame, RectangleShape),
        )
        Box(
            Modifier
                .offset(x = if (checked) 30.dp else 8.dp)
                .size(14.dp, 12.dp)
                .background(block),
        )
    }
}
