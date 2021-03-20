using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class Slot : MonoBehaviour
{
    public SlotStatus Status = SlotStatus.EMPTY;
    public CardRenderer CharacterRenderer;
    public DissolveSphere Pill;

    public ParticleSystem Burst;


    // Update is called once per frame
    void Update()
    {
        switch (Status)
        {
            case SlotStatus.EMPTY:
                Pill.gameObject.SetActive(false);
                CharacterRenderer.gameObject.SetActive(false);
                break;
            case SlotStatus.CHARGED:

                Pill.gameObject.SetActive(true);
                CharacterRenderer.gameObject.SetActive(false);
                break;
            case SlotStatus.OPENING:

                Pill.gameObject.SetActive(true);
                CharacterRenderer.gameObject.SetActive(true);
                if (CharacterRenderer.transform.localScale == Vector3.one)
                    Status = SlotStatus.OPEN;
                break;
            case SlotStatus.OPEN:
                Pill.gameObject.SetActive(false);
                CharacterRenderer.gameObject.SetActive(true);
                break;
            default:
                break;
        }
    }

    [ContextMenu("Drop")]
    public void DropPill()
    {

        Pill.transform.localScale = Vector3.zero;
        Status = SlotStatus.CHARGED;
        LeanTween.scale(Pill.gameObject, Vector3.one, 1).setEase(LeanTweenType.easeOutElastic);
    }

    public void TryUnveil()
    {
        if (Status == SlotStatus.CHARGED)
        {
            LeanTween.value(gameObject, updateValueExampleCallback, 0f, 1f, 0.75f).setEase(LeanTweenType.easeInExpo);
            CharacterRenderer.transform.localScale = Vector3.zero;
            Status = SlotStatus.OPENING;
            Burst.Emit(20);
            LeanTween.scale(CharacterRenderer.gameObject, Vector3.one, 0.75f).setEase(LeanTweenType.easeInExpo);
        }
        if (Status == SlotStatus.OPEN)
        {
            Burst.Emit(20);
            Status = SlotStatus.EMPTY;
            Pill.Slider = 0;
        }
    }

    internal void Fill(Token token)
    {
        CharacterRenderer.ReadToken(token);
        DropPill();
    }

    private void updateValueExampleCallback(float value)
    {
        Pill.Slider = value;
    }

}
