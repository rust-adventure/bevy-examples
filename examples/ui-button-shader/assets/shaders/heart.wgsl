// This shader draws a circle with a given input color
#import bevy_ui::ui_vertex_output::UiVertexOutput
#import bevy_shader_utils::simplex_noise_3d::simplex_noise_3d;

// mesh_view_bindings::globals;

struct CustomUiMaterial {
    @location(0) color: vec4<f32>,
    @location(1) time: f32,
    // 0-1f32
    @location(2) fill_level: f32,
    @location(3) offset: f32
}

@group(1) @binding(0)
var<uniform> input: CustomUiMaterial;

@fragment
fn fragment(in: UiVertexOutput) -> @location(0) vec4<f32> {
    // the UVs are now adjusted around the middle of the rect.
    var uv = in.uv * 2.0 - 1.0;
    uv.y = -uv.y;
// let output = fill_(uv);    
let output = heart_fill(uv);
return output;
    // let distance = sd_heart(uv);
    // let mix_value = step(0.1, distance);
    // let mix_value_2 = step(0.11, distance);

    // let color = mix(
    //     vec4(input.color.rgb, 1.0),
    //     vec4(1.,1.,1.,1.),
    //     mix_value
    // );
    // let color_with_border = mix(
    //     color,
    //     vec4(1.,1.,1.,0.),
    //     mix_value_2,
    // );
    // return color_with_border;
}

// @fragment
// fn fragment(in: UiVertexOutput) -> @location(0) vec4<f32> {
//     // the UVs are now adjusted around the middle of the rect.
//     var uv = in.uv * 2.0 - 1.0;
//     uv.y = -uv.y;
    
//     let distance = sd_heart(uv);
//     let mix_value = step(0.1, distance);
//     let mix_value_2 = step(0.11, distance);

//     let color = mix(
//         vec4(input.color.rgb, 1.0),
//         vec4(1.,1.,1.,1.),
//         mix_value
//     );
//     let color_with_border = mix(
//         color,
//         vec4(1.,1.,1.,0.),
//         mix_value_2,
//     );
//     return color_with_border;

// }

fn sd_heart( point: vec2f ) -> f32
{
    var p = point + vec2(0.,0.5);
    p.x = abs(p.x);

    if( p.y+p.x>1.0 ){
        return sqrt(dot2(p-vec2(0.25,0.75))) - sqrt(2.0)/4.0;
    } else {
    return sqrt(min(dot2(p-vec2(0.00,1.00)),
                    dot2(p - 0.5 * max(p.x+p.y,0.0)))
               ) * sign(p.x-p.y);
    }
}

fn dot2( v: vec2f ) -> f32 {
    return dot(v,v);
}

const PI = 3.14159;

// https://iquilezles.org/articles/distfunctions2d/

fn sd_arc(point: vec2f, sc: vec2f, ra: f32, rb: f32) -> f32
{
    var p = point;
    // sc is the sin/cos of the arc's aperture
    p.x = abs(p.x);
    var a: f32;
    if (sc.y * p.x > sc.x * p.y) {
        a = length(p-sc*ra);
    } else {
        a = abs(length(p)-ra);
    };
    return a - rb;
}

fn get_2d_rotation_matrix(angle: f32) -> mat2x2f
{
    let c: f32 = cos(angle);
    let s: f32 = sin(angle);

    return mat2x2(c, -s, s, c);
}

// https://www.shadertoy.com/view/Ds3fRN
fn fill_(uv_in: vec2f)-> vec4f
{
    let time = 25.;   
    // let aspectRatio: f32 = iResolution.x / iResolution.y;
    // let uv: vec2f = vec2(uv_in.x * aspectRatio, uv_in.y);
    let uv = uv_in;
            
    // Get circle SDF -> clip 3x circles.
            
    let circleSDF: f32 = length(uv);
    
    let ringWidth: f32 = 0.1;
    let innerCircleRadiusOffset: f32 = 0.05;
    
    let oneMinusRingWidth: f32 = 1.0 - ringWidth;
    
    // 2x circles used to generate outer ring.
    
    let circleA: f32 = step(circleSDF, 1.0);
    let circleB: f32 = step(circleSDF, oneMinusRingWidth);
        
    let ring: f32 = circleA - circleB;
    
    // 1x circle used for the actual container/shell (as its mask).
    
    let fillMaskCircle: f32 = step(circleSDF, oneMinusRingWidth - innerCircleRadiusOffset);
    
    // Ring glow.
        
    let ringGlowCircleSDF: f32 = circleSDF - 1.0;
    let innerRingGlowRadiusOffset: f32 = 0.15;
    
    var innerRingGlow: f32 = ringGlowCircleSDF + innerRingGlowRadiusOffset;    
    var outerRingGlow: f32 = ringGlowCircleSDF;   
    
    let outerRingGlowWidth: f32 = 0.01;
    var outerRingGlowPower: f32 = 0.8;
    
    let innerRingGlowWidth: f32 = 0.01;
    let innerRingGlowPower: f32 = 1.2;
    
    let outerRingGlowAnimation: f32 = 12.0;
    let outerRingGlowAnimationRange: f32 = 0.2;    
    
    innerRingGlow = pow(innerRingGlowWidth / innerRingGlow, innerRingGlowPower);
    innerRingGlow = clamp(innerRingGlow - fillMaskCircle, 0.0, 1.0); 
    
    outerRingGlowPower += (sin(time * outerRingGlowAnimation) * outerRingGlowAnimationRange);
    
    outerRingGlow = pow(outerRingGlowWidth / outerRingGlow, outerRingGlowPower);
    outerRingGlow = clamp(outerRingGlow - fillMaskCircle, 0.0, 1.0);
    
    // Progress/fill. Animated.
    
    let fillAnimationFrequency: f32 = 4.0;
    let fillAnimationAmplitude: f32 = 0.05;
        
    let fillAnimationPhase: f32 = time * fillAnimationFrequency;
    
    var fillAnimation: f32 = sin(fillAnimationPhase) * fillAnimationAmplitude;
    
    let waveFrequency: f32 = 2.0;
    let waveAmplitude: f32 = 0.05;
    
    let waveAnimation: f32 = 2.0;
    
    // Waves as repeating sine/band offsets to the horizontal gradient.
    
    var frontWavePhase: f32 = (time * waveAnimation) + uv.x;
    var backWavePhase: f32 = (time * -waveAnimation) + uv.x;
        
    frontWavePhase *= waveFrequency;
    backWavePhase *= waveFrequency;
        
    let backWavesPhaseOffset: f32 = PI;
    
    var frontWaves: f32 = sin(frontWavePhase) * waveAmplitude;
    var backWaves: f32 = sin(backWavePhase + backWavesPhaseOffset) * waveAmplitude;
            
    var verticalBand: f32 = sin(uv.x + (PI * 0.5)) - 0.3;
    verticalBand = smoothstep(0.1, 0.9, verticalBand);
   
    // Stretch waves up/down near center, synced as they bob up/down.
    
    let animatedVerticalBandStrength: f32 = 0.125;
    var animatedVerticalBand: f32 = verticalBand * animatedVerticalBandStrength;
            
    animatedVerticalBand *= sin(time * fillAnimationFrequency);
            
    frontWaves += animatedVerticalBand;
    backWaves -= animatedVerticalBand;
    
    // Pinch sides (mask by the vertical gradient band) so they don't move.
    
    fillAnimation *= verticalBand;
            
    // Centered fill progress.
    // 0.0 = center, -0.5 = bottom, 0.5 = top.
    
    let fillProgressAnimationFrequency: f32 = 1.0;
    let fillProgressAnimationAmplitude: f32 = 0.1;
    
    var fillProgress: f32 = -0.2;
    
    fillProgress += sin((time * fillProgressAnimationFrequency) * PI) * fillProgressAnimationAmplitude;
    //fillProgress = (fillProgress - 0.5) * 2.0; 
    
    var frontFill: f32 = step(uv.y, (fillAnimation + frontWaves) + fillProgress);
    var backFill: f32 = step(uv.y, (-fillAnimation + backWaves) + fillProgress);
    
    frontFill *= fillMaskCircle;
    backFill *= fillMaskCircle;
    
    // Mask back fill to only parts that would be visible separate from frontFill.
    
    backFill = clamp(backFill - frontFill, 0.0, 1.0);
    
    var fillMask: f32 = 1.0 - (frontFill + backFill);
    fillMask *= fillMaskCircle;
        
    let fill: f32 = frontFill + backFill;
    
    // Simple edge glow using radial gradient (circle SDF).
    
    let fresnelOffset: f32 = 0.01;
    var fresnel: f32 = (circleSDF + fresnelOffset) * fillMask;    
    
    let fresnelPower: f32 = 5.0;
    fresnel = clamp(pow(fresnel, fresnelPower), 0.0, 1.0);
    
    let frontFillFresnelPower: f32 = 5.0;
    let frontFillFresnelOffset: f32= 0.02;
    
    var frontFillFresnel: f32 = (circleSDF + frontFillFresnelOffset) * (1.0 - fillMask);
    frontFillFresnel = clamp(pow(frontFillFresnel, frontFillFresnelPower), 0.0, 1.0);
    
    // Specular reflection, drawn (stylized, like a cartoon) as two arcs.
        
    let specularArcAngle1: f32 = radians(15.0);
    let specularArcAngle2: f32 = radians(2.0);
    
    let specularArcRotation1: f32 = radians(60.0);
    let specularArcRotation2: f32 = radians(28.0);
    
    let specularArcSC1: vec2f = vec2(sin(specularArcAngle1), cos(specularArcAngle1));
    let specularArcSC2: vec2f = vec2(sin(specularArcAngle2), cos(specularArcAngle2));
    
    let specularArcOffset: f32 = 0.35;
    let specularArcWidth: f32 = 0.07;
    
    let specularArcUV1: vec2f = get_2d_rotation_matrix(specularArcRotation1) * uv;
    let specularArcUV2: vec2f = get_2d_rotation_matrix(specularArcRotation2) * uv;
    
    var specularArc1: f32 = sd_arc(specularArcUV1, specularArcSC1, 1.0 - specularArcOffset, specularArcWidth);
    var specularArc2: f32 = sd_arc(specularArcUV2, specularArcSC2, 1.0 - specularArcOffset, specularArcWidth);
        
    specularArc1 = step(specularArc1, 0.0);
    specularArc2 = step(specularArc2, 0.0);
    
    let specularStrength: f32 = 0.2;
    var specular: f32 = specularArc1 + specularArc2;
    
    specular *= specularStrength;
    
    // Final mask. Can be used as alpha.
    
    let mask: f32 = ring + fill + fresnel + specular;
    
    // Per-mask RGB colour.
    
    let ringColour: vec3f = vec3(1.0, 0.9, 0.8);
    
    let frontFillInnerColour: vec3f = vec3(1.0, 0.2, 0.1);
    let frontFillOuterColour: vec3f = vec3(0.0, 0.0, 0.0);
    
    let frontFillColour: vec3f = mix(frontFillInnerColour, frontFillOuterColour, frontFillFresnel);
    
    let backFillColour: vec3f = vec3(0.5, 0.0, 0.0);
    
    let specularColour: vec3f = vec3(1.0, 1.0, 0.9);
    let fresnelColour: vec3f = vec3(0.5, 0.0, 0.3);
    
    let innerRingGlowColour: vec3f = vec3(1.0, 0.3, 0.1);
    let outerRingGlowColour: vec3f = vec3(1.0, 0.8, 0.1);
             
    var rgb: vec3f =
    
        (ring * ringColour) +
        
        (innerRingGlow * innerRingGlowColour) +
        (outerRingGlow * outerRingGlowColour) +
        
        (frontFill * frontFillColour) +
        (backFill * backFillColour) +
        (fresnel * fresnelColour) +
        (specular * specularColour);
    
    // Background gradient. Just for presentation.
    
    let backgroundGradientPower: f32 = 0.6;
    
    var backgroundGradient: f32 = length(uv);
    
    backgroundGradient = pow(backgroundGradient, backgroundGradientPower);
    backgroundGradient = smoothstep(0.0, 1.0, backgroundGradient);
    
    let backgroundGradientInnerColour: vec3f = vec3(0.13, 0.0, 0.4);
    let backgroundGradientOuterColour: vec3f = vec3(0.0, 0.0, 0.0);
    
    var background: vec3f = mix(backgroundGradientInnerColour, backgroundGradientOuterColour, backgroundGradient);
    
    // Simply add the background to the composite so far.
    background = clamp(background - (fill + ring), vec3(0.0), vec3(1.0));
         
    let backgroundStrength: f32 = 0.65;    
    background *= backgroundStrength;
    
    rgb += background;
    
    return vec4(rgb, mask);
}

fn heart_fill(uv_in: vec2f)-> vec4f
{
    // let time = 25.;
    let time = (input.time / 2.0) + input.offset;
    // let aspectRatio: f32 = iResolution.x / iResolution.y;
    // let uv: vec2f = vec2(uv_in.x * aspectRatio, uv_in.y);
    let uv = uv_in;
            
    // Get circle SDF -> clip 3x circles.
            
    // let circleSDF: f32 = length(uv);
    // * 4 is important for heart
    let circleSDF: f32 = sd_heart(uv)*4.0;
    
    let ringWidth: f32 = 0.1;
    let innerCircleRadiusOffset: f32 = 0.05;
    
    let oneMinusRingWidth: f32 = 1.0 - ringWidth;
    
    // 2x circles used to generate outer ring.
    
    let circleA: f32 = step(circleSDF, 1.0);
    let circleB: f32 = step(circleSDF, oneMinusRingWidth);
        
    let ring: f32 = circleA - circleB;
    
    // 1x circle used for the actual container/shell (as its mask).
    
    let fillMaskCircle: f32 = step(circleSDF, oneMinusRingWidth - innerCircleRadiusOffset);
    
    // Ring glow.
        
    let ringGlowCircleSDF: f32 = circleSDF - 1.0;
    let innerRingGlowRadiusOffset: f32 = 0.15;
    
    var innerRingGlow: f32 = ringGlowCircleSDF + innerRingGlowRadiusOffset;    
    var outerRingGlow: f32 = ringGlowCircleSDF;   
    
    let outerRingGlowWidth: f32 = 0.01;
    var outerRingGlowPower: f32 = 0.8;
    
    let innerRingGlowWidth: f32 = 0.01;
    let innerRingGlowPower: f32 = 1.2;
    
    let outerRingGlowAnimation: f32 = 12.0;
    let outerRingGlowAnimationRange: f32 = 0.2;    
    
    innerRingGlow = pow(innerRingGlowWidth / innerRingGlow, innerRingGlowPower);
    innerRingGlow = clamp(innerRingGlow - fillMaskCircle, 0.0, 1.0); 
    
    outerRingGlowPower += (sin(time * outerRingGlowAnimation) * outerRingGlowAnimationRange);
    
    outerRingGlow = pow(outerRingGlowWidth / outerRingGlow, outerRingGlowPower);
    outerRingGlow = clamp(outerRingGlow - fillMaskCircle, 0.0, 1.0);
    
    // Progress/fill. Animated.
    
    let fillAnimationFrequency: f32 = 4.0;
    let fillAnimationAmplitude: f32 = 0.05;
        
    let fillAnimationPhase: f32 = time * fillAnimationFrequency;
    
    var fillAnimation: f32 = sin(fillAnimationPhase) * fillAnimationAmplitude;
    
    let waveFrequency: f32 = 2.0;
    let waveAmplitude: f32 = 0.05;
    
    let waveAnimation: f32 = 2.0;
    
    // Waves as repeating sine/band offsets to the horizontal gradient.
    
    var frontWavePhase: f32 = (time * waveAnimation) + uv.x;
    var backWavePhase: f32 = (time * -waveAnimation) + uv.x;
        
    frontWavePhase *= waveFrequency;
    backWavePhase *= waveFrequency;
        
    let backWavesPhaseOffset: f32 = PI;
    
    var frontWaves: f32 = sin(frontWavePhase) * waveAmplitude;
    var backWaves: f32 = sin(backWavePhase + backWavesPhaseOffset) * waveAmplitude;
            
    var verticalBand: f32 = sin(uv.x + (PI * 0.5)) - 0.3;
    verticalBand = smoothstep(0.1, 0.9, verticalBand);
   
    // Stretch waves up/down near center, synced as they bob up/down.
    
    let animatedVerticalBandStrength: f32 = 0.125;
    var animatedVerticalBand: f32 = verticalBand * animatedVerticalBandStrength;
            
    animatedVerticalBand *= sin(time * fillAnimationFrequency);
            
    frontWaves += animatedVerticalBand;
    backWaves -= animatedVerticalBand;
    
    // Pinch sides (mask by the vertical gradient band) so they don't move.
    
    fillAnimation *= verticalBand;
            
    // Centered fill progress.
    // 0.0 = center, -0.5 = bottom, 0.5 = top.
    
    let fillProgressAnimationFrequency: f32 = 1.0;
    let fillProgressAnimationAmplitude: f32 = 0.1;
    
    var fillProgress: f32 = input.fill_level;
    
    fillProgress += sin((time * fillProgressAnimationFrequency) * PI) * fillProgressAnimationAmplitude;
    //fillProgress = (fillProgress - 0.5) * 2.0; 
    
    var frontFill: f32 = step(uv.y, (fillAnimation + frontWaves) + fillProgress);
    var backFill: f32 = step(uv.y, (-fillAnimation + backWaves) + fillProgress);
    
    frontFill *= fillMaskCircle;
    backFill *= fillMaskCircle;
    
    // Mask back fill to only parts that would be visible separate from frontFill.
    
    backFill = clamp(backFill - frontFill, 0.0, 1.0);
    
    var fillMask: f32 = 1.0 - (frontFill + backFill);
    fillMask *= fillMaskCircle;
        
    let fill: f32 = frontFill + backFill;
    
    // Simple edge glow using radial gradient (circle SDF).
    
    let fresnelOffset: f32 = 0.01;
    var fresnel: f32 = (circleSDF + fresnelOffset) * fillMask;    
    
    let fresnelPower: f32 = 5.0;
    fresnel = clamp(pow(fresnel, fresnelPower), 0.0, 1.0);
    
    let frontFillFresnelPower: f32 = 5.0;
    let frontFillFresnelOffset: f32= 0.02;
    
    var frontFillFresnel: f32 = (circleSDF + frontFillFresnelOffset) * (1.0 - fillMask);
    frontFillFresnel = clamp(pow(frontFillFresnel, frontFillFresnelPower), 0.0, 1.0);
    
    // Specular reflection, drawn (stylized, like a cartoon) as two arcs.
        
    let specularArcAngle1: f32 = radians(15.0);
    let specularArcAngle2: f32 = radians(2.0);
    
    let specularArcRotation1: f32 = radians(60.0);
    let specularArcRotation2: f32 = radians(28.0);
    
    let specularArcSC1: vec2f = vec2(sin(specularArcAngle1), cos(specularArcAngle1));
    let specularArcSC2: vec2f = vec2(sin(specularArcAngle2), cos(specularArcAngle2));
    
    let specularArcOffset: f32 = 0.35;
    let specularArcWidth: f32 = 0.07;
    
    let specularArcUV1: vec2f = get_2d_rotation_matrix(specularArcRotation1) * uv;
    let specularArcUV2: vec2f = get_2d_rotation_matrix(specularArcRotation2) * uv;
    
    var specularArc1: f32 = sd_arc(specularArcUV1, specularArcSC1, 1.0 - specularArcOffset, specularArcWidth);
    var specularArc2: f32 = sd_arc(specularArcUV2, specularArcSC2, 1.0 - specularArcOffset, specularArcWidth);
        
    specularArc1 = step(specularArc1, 0.0);
    specularArc2 = step(specularArc2, 0.0);
    
    let specularStrength: f32 = 0.2;
    var specular: f32 = specularArc1 + specularArc2;
    
    specular *= specularStrength;
    
    // Final mask. Can be used as alpha.
    // all_filler is custom
    var all_filler: f32 = 0.;
    if (circleSDF<1.) {
        all_filler = 1.;
    };
    let mask: f32 = ring + fill + fresnel + specular + all_filler;
    // Per-mask RGB colour.
    
    // let ringColour: vec3f = vec3(1.0, 0.9, 0.8);
    let ringColour: vec3f = vec3(0.33, 0.29, 0.35);
    
    // let frontFillInnerColour: vec3f = vec3(1.0, 0.2, 0.1);
    // let frontFillOuterColour: vec3f = vec3(0.0, 0.0, 0.0);
    let frontFillInnerColour: vec3f = vec3(1.0, 0.49, 0.42);
    let frontFillOuterColour: vec3f = vec3(0.0, 0.0, 0.0);
    
    let frontFillColour: vec3f = mix(frontFillInnerColour, frontFillOuterColour, frontFillFresnel);
    
    // let backFillColour: vec3f = vec3(0.5, 0.0, 0.0);
    let backFillColour: vec3f = vec3(1., 0.36, 0.40);
    
    let specularColour: vec3f = vec3(1.0, 1.0, 0.9);
    let fresnelColour: vec3f = vec3(0.5, 0.0, 0.3);
    
    let innerRingGlowColour: vec3f = vec3(1.0, 0.3, 0.1);
    let outerRingGlowColour: vec3f = vec3(1.0, 0.2, 0.1);

        var ring_glow_conditional = 
        (innerRingGlow * innerRingGlowColour) +
        (outerRingGlow * outerRingGlowColour);

        // fill_level = -1..1
        if (input.fill_level) > -0.25 {
            ring_glow_conditional = vec3(0.);
        };
        var rgb: vec3f =
    
        (ring * ringColour) +
        
        // (innerRingGlow * innerRingGlowColour) +
        // (outerRingGlow * outerRingGlowColour) +
        ring_glow_conditional +
        
        (frontFill * frontFillColour) +
        (backFill * backFillColour) +
        (fresnel * fresnelColour) +
        (specular * specularColour);
    
    // Background gradient. Just for presentation.
    
    let backgroundGradientPower: f32 = 0.6;
    
    var backgroundGradient: f32 = length(uv);
    
    backgroundGradient = pow(backgroundGradient, backgroundGradientPower);
    backgroundGradient = smoothstep(0.0, 1.0, backgroundGradient);
    
    // let backgroundGradientInnerColour: vec3f = vec3(0.13, 0.0, 0.4);
    // let backgroundGradientOuterColour: vec3f = vec3(0.0, 0.0, 0.0);
    let backgroundGradientInnerColour: vec3f = vec3(0.33, 0.29, 0.35);
    let backgroundGradientOuterColour: vec3f = vec3(0.33, 0.29, 0.35);
    
    var background: vec3f = mix(backgroundGradientInnerColour, backgroundGradientOuterColour, backgroundGradient);
    
    // Simply add the background to the composite so far.
    background = clamp(background - (fill + ring), vec3(0.0), vec3(1.0));
         
    let backgroundStrength: f32 = 0.65;    
    background *= backgroundStrength;

    if (circleSDF < 1.0) {
        rgb += background;
    }


    // if (rgb.b < 0.01) {
    //     return vec4(rgb, 0.);
    // } else {
        return vec4(rgb, mask);
    // }
    // return vec4(rgb, 1.);

}