using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class SelectionRooster : MonoBehaviour
{

    public Camera SectionCamera;
    public Transform[] FixPoints;

    public CardRenderer[] CardHolders;

    public float Duration=1f;

    private int CurrentCenter;
    private int WheelOffset;

    private Token[] Characters = new Token[10];


    private void Start()
    {
        for (int i = 0; i < Characters.Length; i++)
        {
            Characters[i] = new Token() { biotech = i, engineering = i, psychics = i, weapons =i, name = "" };
        }

        CurrentCenter = 0;
        WheelOffset = 0;
        UpdatePositions(0,true);
        for (int i = 0; i < FixPoints.Length; i++)
        {
            RotateLeft();
        }
    }

    [ContextMenu("Right")]
    public void RotateRight()
    {
        CurrentCenter = (CurrentCenter + 1 + Characters.Length) % Characters.Length;
        WheelOffset = (WheelOffset + 1) % FixPoints.Length;
        UpdatePositions(Duration,true);
    }

    [ContextMenu("Left")]
    public void RotateLeft()
    {
        CurrentCenter = (CurrentCenter - 1 + Characters.Length) % Characters.Length;

        WheelOffset = (WheelOffset - 1) % FixPoints.Length;
        UpdatePositions(Duration,false);
    }


    public void UpdatePositions(float duration, bool increase)
    {
        for (int i = 0; i < CardHolders.Length; i++)
        {
            var newPos = i + WheelOffset + FixPoints.Length;
            var sanitized = newPos % FixPoints.Length;
            if ((increase && sanitized == 0) || (!increase && sanitized == FixPoints.Length - 1))
            {
                CardHolders[i].transform.position = FixPoints[sanitized].position;
                CardHolders[i].ReadToken(sanitized == 0 ? Characters[(0 + CurrentCenter + CardHolders.Length-1)%Characters.Length] : Characters[0 + CurrentCenter]);
            }
            else
                LeanTween.move(CardHolders[i].gameObject, FixPoints[sanitized], duration).setEase(LeanTweenType.easeInOutExpo);
            LeanTween.rotate(CardHolders[i].gameObject, FixPoints[sanitized].rotation.eulerAngles, duration);
        }


    }
}

