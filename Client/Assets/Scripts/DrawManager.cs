using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public enum SlotStatus
{
    EMPTY,
    CHARGED,
    OPENING,
    OPEN
}
public class DrawManager : MonoBehaviour
{
    public Camera DrawCamera;
    public Slot[] Slots;
    private Stack<Token> _tokensToDrop = new Stack<Token>();
    public float TimeOut = 0;
    public TMPro.TMP_Text PillCount;

    private int oldSlot = 0;

    public void PrepareSlots(Token[] tokens)
    {
        for (int i = 0; i < tokens.Length; i++)
        {
            _tokensToDrop.Push(tokens[i]);
        }
    }

    public void PrepareSlot(Token token)
    {
        if (oldSlot > 2) return;
        _tokensToDrop.Push(token);
    }

    private string paddedNumbers(int number)
    {
        if (number < 10)
            return "00" + number.ToString();

        if (number < 100)
            return "0" + number.ToString();
        return number.ToString();
    }

    private void Update()
    {
        //if (oldSlot > 2) return;

        PillCount.text = paddedNumbers(_tokensToDrop.Count);

        bool allFilled = true;
        foreach (var item in Slots)
        {
            if (item.Status == SlotStatus.EMPTY)
            {
                allFilled = false;
                break;
            }
        }
       
        if (!allFilled&&_tokensToDrop.Count > 0)
        {
            TimeOut = Mathf.Max(0, TimeOut - Time.deltaTime);
            if (TimeOut <= 0)
            {
                var slot = Random.Range(0, 3); oldSlot++;
                if (Slots[slot].Status == SlotStatus.EMPTY)
                {
                    Slots[slot].Fill(_tokensToDrop.Pop());
                    TimeOut = Random.Range(1, 2) + Random.value;
                }
            }
        }

        // PillCount.text = paddedNumbers(oldSlot);
    }
}
