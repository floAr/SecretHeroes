using System;
using UnityEngine;

public class FadeCamera : MonoBehaviour
{
    public AnimationCurve FadeCurve;
    private float _alpha = 1;
    [Range(0, 1)]
    public float Target = 0;
    public float Duration = 1f;
    private Texture2D _texture;
    public LeanTweenType Easing;
    public Color FadeColor;


    public void Reset()
    {
        _alpha = 1;

    }

    private void Start()
    {
        FadeIn();
    }
    [RuntimeInitializeOnLoadMethod]
    public void RedoFade()
    {
        Reset();
    }

    [ContextMenu("Fade Out")]
    public void FadeOut()
    {
        LeanTween.value(gameObject, updateValueExampleCallback, 1f, 0f, Duration).setEase(Easing);
    }

    [ContextMenu("Fade In")]
    public void FadeIn()
    {
        LeanTween.value(gameObject, updateValueExampleCallback, 0f, 1f, Duration).setEase(Easing);
    }

    private void updateValueExampleCallback(float value)
    {
        Target = value;
    }

    public void OnGUI()
    {

        if (_texture == null) _texture = new Texture2D(1, 1);

        FadeColor.a = _alpha;
        _texture.SetPixel(0, 0, FadeColor);
        _texture.Apply();


        _alpha = FadeCurve.Evaluate(Target);
        GUI.DrawTexture(new Rect(0, 0, Screen.width, Screen.height), _texture);

    }
}