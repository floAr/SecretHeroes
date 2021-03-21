using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class SelectionRooster : MonoBehaviour
{

    public Camera SectionCamera;
    public Transform[] FixPoints;

    public CardRenderer[] CardHolders;

    public float Duration = 1f;

    private int CurrentCenter;
    private int WheelOffset;

    private Rooster Rooster;

    //private Token[] Characters = new Token[10];

    public void Refresh()
    {
        var duration = Duration;
        Duration = 0;
        if (Rooster.MyHeroes.Count > 0)
        {
            for (int i = 0; i < FixPoints.Length; i++)
            {
                RotateLeft();
            }
            for (int i = 0; i < FixPoints.Length; i++)
            {
                RotateRight();
            }
        }
        Duration = duration;
    }

    private void Start()
    {

        Rooster = GameObject.FindObjectOfType<Rooster>();

        CurrentCenter = 0;
        WheelOffset = 0;
        if (Rooster.MyHeroes.Count > 0)
        {
            UpdatePositions(0, true);
            for (int i = 0; i < FixPoints.Length; i++)
            {
                RotateLeft();
            }
        }
        
    }

    [ContextMenu("Right")]
    public void RotateRight()
    {
        if (Rooster.MyHeroes.Count == 0) return;
        CurrentCenter = (CurrentCenter + 1 + Rooster.MyHeroes.Count) % Rooster.MyHeroes.Count;
        WheelOffset = (WheelOffset + 1) % FixPoints.Length;
        UpdatePositions(Duration, true);
    }

    [ContextMenu("Left")]
    public void RotateLeft()
    {
        if (Rooster.MyHeroes.Count == 0) return;
        CurrentCenter = (CurrentCenter - 1 + Rooster.MyHeroes.Count) % Rooster.MyHeroes.Count;

        WheelOffset = (WheelOffset - 1) % FixPoints.Length;
        UpdatePositions( Duration , true);
    }


    public void UpdatePositions(float duration, bool increase)
    {
        if (Rooster.MyHeroes.Count == 0) return;
        for (int i = 0; i < CardHolders.Length; i++)
        {
            var newPos = i + WheelOffset + FixPoints.Length;
            var sanitized = newPos % FixPoints.Length;
            if ((increase && sanitized == 0) || (!increase && sanitized == FixPoints.Length - 1))
            {
                CardHolders[i].transform.position = FixPoints[sanitized].position;
                CardHolders[i].ReadToken(sanitized == 0 ? Rooster.MyHeroes[(0 + CurrentCenter + CardHolders.Length - 1) % Rooster.MyHeroes.Count] : Rooster.MyHeroes[0 + CurrentCenter]);

            }
            else
                LeanTween.move(CardHolders[i].gameObject, FixPoints[sanitized], duration).setEase(LeanTweenType.easeInOutExpo);

            CardHolders[i].SetSelected(sanitized == 2);

            LeanTween.rotate(CardHolders[i].gameObject, FixPoints[sanitized].rotation.eulerAngles, duration);


        }


    }
}

