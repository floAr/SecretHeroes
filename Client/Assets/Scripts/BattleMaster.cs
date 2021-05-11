using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UI;

public class BattleMaster : MonoBehaviour
{

    public Camera BattleCamera;

    public BattleSlot MyHero;
    public BattleSlot Enemy1;
    public BattleSlot Enemy2;


    public Button SendButton;


    public void OptimisticResponse(Token hero)
    {
        MyHero.Activate(hero);
    }

    public void AddBattleState(BattleState state)
    {
        MyHero.Deactivate();
        Enemy1.Deactivate();
        Enemy2.Deactivate();
        SendButton.interactable = true;
        int counter = 0;
        if (state.your_hero != null)
        {

            try
            {
                var name = state.your_hero.name;
                if (name.Length > 0)
                {
                    counter += 1;
                    MyHero.Activate(state.your_hero);
                    SendButton.interactable = false;

                }
            }
            catch (Exception e) { }
        }
        var other = state.heroes_waiting - counter;
        if (other==1)
        {
            if (UnityEngine.Random.value < 0.5f)
            {
                Enemy1.Activate(null);
            }
            else
            {
                Enemy2.Activate(null);
            }

        }
        if (other == 2)
        {
            Enemy1.Activate(null);
            Enemy2.Activate(null);
        }

    }

    [ContextMenu("Test single")]
    public void testSingle()
    {
        AddBattleState(new BattleState() { heroes_waiting = 1, your_hero = Token.Random() });
    }

    [ContextMenu("Test 2")]
    public void testTwo()
    {
        AddBattleState(new BattleState() { heroes_waiting = 2, your_hero = Token.Random() });
    }
}
